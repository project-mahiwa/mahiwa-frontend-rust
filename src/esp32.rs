#[link(wasm_import_module = "esp32")]
extern "C" {
    #[link_name = "esp32random"]
    fn _random() -> i64;
}

/// hardware random number generator
///
/// # Examples
///
/// ```
/// let result = esp32::random();
/// ```
pub fn random() -> i64 {
    unsafe { _random() }
}
