#[link(wasm_import_module = "serial")]
extern "C" {
    #[link_name = "print"]
    fn _print(utf8: *const u8, len: usize);
}

/// Equivalent to Arduino's print
///
/// # Examples
///
/// ```
/// serial::print("hello mahiwa");
/// ```
pub fn print(string: &str) {
    unsafe { _print(string.as_ptr(), string.len()) }
}
/// Equivalent to Arduino's println
pub fn println(string: &str) {
    print(string);
    print("\n");
}
