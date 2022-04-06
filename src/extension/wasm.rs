use wasmer::{imports, Instance, Module, Store};
/// import wasm extension
pub fn wasm_import(source: &str) -> anyhow::Result<Instance> {
    let store = Store::default();
    let module = Module::new(&store, source)?;
    let import_object = imports!();
    Instance::new(&module, &import_object).map_err(|e| anyhow::anyhow!(e))
}
