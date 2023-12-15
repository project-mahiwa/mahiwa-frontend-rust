#[link(wasm_import_module = "serial")]
extern "C" {
    #[link_name = "print"]
    fn _print(utf8: *const u8, len: usize);
    #[link_name = "printInt"]
    fn _printInt(num: i32);
    #[link_name = "printLong"]
    fn _printLong(num: i64);
    #[link_name = "printFloat"]
    fn _printFloat(num: f32, format: u8);
    #[link_name = "printDouble"]
    fn _printDouble(num: f64, format: u8);
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

pub fn print_int(num: i32) {
    unsafe { _printInt(num) }
}
pub fn print_long(num: i64) {
    unsafe { _printLong(num) }
}
pub fn print_float(num: f32, format: u8) {
    unsafe { _printFloat(num, format) }
}
pub fn print_double(num: f64, format: u8) {
    unsafe { _printDouble(num, format) }
}

/// Equivalent to Arduino's println
pub fn println(string: &str) {
    print(string);
    print("\n");
}
