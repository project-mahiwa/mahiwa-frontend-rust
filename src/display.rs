#[link(wasm_import_module = "display")]
extern "C" {
    #[link_name = "init"]
    fn _init();
    #[link_name = "printD"]
    fn _print(utf8: *const u8, len: usize);
}

/// initialize display
///
/// # Examples
///
/// ```
/// display::init();
/// ```
pub fn init() {
    unsafe { _init() }
}

/// print to display
///
/// # Examples
///
/// ```
/// display::print("hello mahiwa");
/// ```
pub fn print(string: &str) {
    unsafe { _print(string.as_ptr(), string.len()) }
}
