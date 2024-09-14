use std::{process::Command, str::FromStr};

use anyhow::Context;
use cargo_toml::Manifest;
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Deploy a script written for LOTUS.
#[derive(Parser)]
pub struct DeployCommand {
    /// The profile to build with.
    #[clap(long, default_value = "dev")]
    profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScriptConfig {
    deploy: ScriptDeployConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ScriptDeployConfig {
    package: Option<String>,
    user_id: i32,
    sub_id: i32,
}

impl ScriptConfig {
    fn from_path(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            anyhow::bail!("Script config file does not exist: {}", path.display());
        }

        let content = std::fs::read_to_string(path).context("failed to read script config")?;
        let config = toml::from_str(&content).context("failed to parse script config")?;
        Ok(config)
    }
}

#[derive(Debug, Clone, Copy)]
enum Profile {
    Dev,
    Release,
}

impl Profile {
    fn target_name(&self) -> &'static str {
        match self {
            Profile::Dev => "debug",
            Profile::Release => "release",
        }
    }

    fn profile_name(&self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::Release => "release",
        }
    }
}

impl FromStr for Profile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "release" => Ok(Profile::Release),
            "dev" => Ok(Profile::Dev),
            _ => Err("invalid profile, must be 'release' or 'dev'"),
        }
    }
}

impl DeployCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        let config = ScriptConfig::from_path("./script.toml")?;

        let manifest = Manifest::from_path("./Cargo.toml").context("failed to read Cargo.toml")?;

        let content_dir = content_dir(config.deploy.user_id, config.deploy.sub_id);

        if !content_dir.exists() {
            tracing::info!("Creating content directory");
            std::fs::create_dir_all(&content_dir).with_context(|| {
                format!(
                    "Failed to create content directory: {}",
                    content_dir.display()
                )
            })?;
        }

        let package = match manifest.workspace {
            Some(workspace) => {
                let package = config
                    .deploy
                    .package
                    .as_ref()
                    .context("in a workspace, package name is required")?;
                if !workspace.members.contains(package) {
                    anyhow::bail!("Package {} is not a member of the workspace", package);
                }

                package.as_str()
            }
            None => manifest
                .package
                .as_ref()
                .map(|p| p.name())
                .context("failed to get package name")?,
        };

        build_script(package, self.profile.profile_name()).context("failed to build script")?;

        let target_dir = cargo_target_dir();
        let package_file_name = package.replace("-", "_");
        let wasm_file_path = target_dir
            .join("wasm32-unknown-unknown")
            .join(self.profile.target_name())
            .join(format!("{package_file_name}.wasm"));

        let target_wasm_file_path = content_dir.join(format!("{package_file_name}.wasm"));

        tracing::info!("Copying wasm file");
        std::fs::copy(&wasm_file_path, &target_wasm_file_path).with_context(|| {
            format!(
                "Failed to copy wasm file from {} to {}",
                wasm_file_path.display(),
                target_wasm_file_path.display()
            )
        })?;

        Ok(())
    }
}

fn build_script(package: &str, profile: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--profile")
        .arg(profile)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--package")
        .arg(package);

    execute_command(cmd)
}

fn content_dir(user_id: i32, sub_id: i32) -> std::path::PathBuf {
    let mut path = lotus_data_dir();
    path.push("overrides");
    path.push(user_id.to_string());
    path.push(sub_id.to_string());
    path
}

fn cargo_target_dir() -> std::path::PathBuf {
    std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string())
        .into()
}

fn execute_command(mut cmd: Command) -> anyhow::Result<()> {
    cmd.stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit());

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }

    Ok(())
}

fn lotus_data_dir() -> std::path::PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("LOTUS-Simulator");
    path
}
