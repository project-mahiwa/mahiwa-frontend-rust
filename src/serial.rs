#[link(wasm_import_module = "serial")]
extern "C" {
    #[link_name = "print"]
    fn _print(utf8: *const u8, len: usize);
}

pub fn print(string: &str) {
    unsafe { _print(string.as_ptr(), string.len()) }
}
pub fn println(string: &str) {
    print(string);
    print("\n");
}
