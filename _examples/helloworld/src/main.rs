#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる

use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::print("hello mahiwa written in Rust");
    loop {
        arduino::delay(900);
        serial::print("hello mahiwa");
    }
}
