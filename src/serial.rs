#[link(wasm_import_module = "serial")]
extern "C" {
    #[link_name = "print"]
    fn _print(utf8: *const u8, len: usize);
    #[link_name = "printInt"]
    fn _printInt(int_num: i32);
    #[link_name = "printDouble"]
    fn _printDouble(double_num: f64);
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

pub fn print_int(int_num: i32) {
    unsafe { _printInt(int_num) }
}
pub fn print_double(double_num: f64) {
    unsafe { _printDouble(double_num) }
}

/// Equivalent to Arduino's println
pub fn println(string: &str) {
    print(string);
    print("\n");
}
