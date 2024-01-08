#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::display;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::println("Rust Display with Mahiwa");
    display::init();
    display::print("Rust Test Display!");
}
