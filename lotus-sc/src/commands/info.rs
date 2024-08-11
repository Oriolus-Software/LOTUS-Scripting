use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use wasmtime::Engine;

/// Get info of a compiled lotus-script
#[derive(Parser)]
pub struct InfoCommand {
    /// Path to the lotus-script WASM file
    #[clap(long)]
    path: String,
}

#[derive(Serialize, Deserialize)]
struct InfoOutput {
    vars: Vec<(String, String)>,
}

impl InfoCommand {
    pub fn execute(&self) -> anyhow::Result<()> {
        let engine = Engine::default();
        let module = wasmtime::Module::from_file(&engine, &self.path)
            .with_context(|| format!("failed to load module from file '{}'", self.path))?;

        let mut linker = wasmtime::Linker::new(&engine);

        linker
            .define_unknown_imports_as_default_values(&module)
            .context("failed to define unknown imports as default values")?;

        let mut store = wasmtime::Store::new(&engine, ());
        let instance = linker
            .instantiate(&mut store, &module)
            .with_context(|| format!("failed to instantiate module '{}'", self.path))?;

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "public_vars")
            .context("failed to find public_vars function")?;

        let ptr = func
            .call(&mut store, ())
            .context("failed to call public_vars function")?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .context("failed to find memory export")?;

        let data = &memory.data(&store)[ptr as usize..][..8];
        let len = i32::from_le_bytes(data[0..4].try_into().unwrap());
        let ptr = i32::from_le_bytes(data[4..].try_into().unwrap());

        let data = &memory.data(&store)[ptr as usize..][..len as usize];
        let vars: Vec<(String, String)> =
            rmp_serde::from_read(data).context("failed to deserialize public_vars")?;

        let info_output = InfoOutput { vars };

        println!("{}", serde_json::to_string_pretty(&info_output)?);

        Ok(())
    }
}
