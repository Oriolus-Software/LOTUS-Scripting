use clap::Parser;

mod deploy;
#[cfg(feature = "wasm")]
mod info;

#[derive(Parser)]
pub enum Commands {
    #[cfg(feature = "wasm")]
    Info(info::InfoCommand),
    Deploy(deploy::DeployCommand),
}

impl Commands {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "wasm")]
            Commands::Info(cmd) => cmd.execute(),
            Commands::Deploy(cmd) => cmd.execute(),
        }
    }
}
