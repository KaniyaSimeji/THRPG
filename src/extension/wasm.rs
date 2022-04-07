use wasmer::{imports, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

/// import wasm extension
pub fn wasm_import(source: &str) -> anyhow::Result<Instance> {
    let store = Store::default();
    let module = Module::new(&store, source)?;
    let import_object = imports!();
    Instance::new(&module, &import_object).map_err(|e| anyhow::anyhow!(e))
}

/// compile wasm
pub async fn wasm_init() -> anyhow::Result<()> {
    tokio::spawn(async {
        let compiler = Cranelift::default();
        Store::new(&Universal::new(compiler).engine());
    })
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
