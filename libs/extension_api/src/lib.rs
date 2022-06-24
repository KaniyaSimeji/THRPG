pub mod thrpg_sys;

pub fn print(source: &str) {
    unsafe { thrpg_sys::print(source.as_ptr()) }
}

pub trait EventHandler {
    fn setup() {
        ()
    }
}
