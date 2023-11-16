#[link(wasm_import_module = "arduino")]
extern "C" {
    #[link_name = "delay"]
    fn _delay(ms: u32);
}
pub fn delay(ms: u32) {
    unsafe {
        _delay(ms);
    }
}
