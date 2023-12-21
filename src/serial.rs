#[link(wasm_import_module = "serial")]
extern "C" {
    #[link_name = "print"]
    fn _print(utf8: *const u8, len: usize);
    #[link_name = "printI32"]
    fn _printI32(num: i32);
    #[link_name = "printI64"]
    fn _printI64(num: i64);
    #[link_name = "printF32"]
    fn _printF32(num: f32, format: u8);
    #[link_name = "printF64"]
    fn _printF64(num: f64, format: u8);
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

pub fn print_i32(num: i32) {
    unsafe { _printI32(num) }
}
pub fn print_i64(num: i64) {
    unsafe { _printI64(num) }
}
pub fn print_f32(num: f32, format: u8) {
    unsafe { _printF32(num, format) }
}
pub fn print_f64(num: f64, format: u8) {
    unsafe { _printF64(num, format) }
}

/// Equivalent to Arduino's println
pub fn println(string: &str) {
    print(string);
    print("\n");
}
