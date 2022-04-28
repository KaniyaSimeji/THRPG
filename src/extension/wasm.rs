use wasmer::{imports, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmer_runtime::{Array, Ctx, WasmPtr};

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

pub fn wasmptr_to_str(ctx: &mut Ctx, ptr: u32, len: u32) -> Option<&str> {
    let ctx_memory = ctx.memory(0);
    let wasmptr: WasmPtr<u8, Array> = WasmPtr::new(ptr);
    let wasm_str = wasmptr.get_utf8_string(ctx_memory, len);
    wasm_str
}
