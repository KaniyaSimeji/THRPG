use once_cell::sync::Lazy;
use wasmer::{imports, Function, ImportObject, Instance};

pub static IMPORT_OBJECTS: Lazy<ImportObject> = Lazy::new(|| imports! {});

pub fn seach_function<'a>(
    instance: &'a Instance,
    name: &'a str,
) -> Option<(&'a String, &'a Function)> {
    instance
        .exports
        .iter()
        .functions()
        .filter(|(x, _)| x == &name)
        .nth(0)
}
