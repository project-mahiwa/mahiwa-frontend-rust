#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::esp32;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::println("Rust ESP32 random with Mahiwa");
    loop {
        let random_val: i64 = esp32::random();
        serial::print("Hardware Random Gen : ");
        serial::print_i64(random_val);
        serial::println("");
        arduino::delay(900);
    }
}
