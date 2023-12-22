#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::{arduino, serial};

#[no_mangle]
fn _start() {
    arduino::delay(5000);
    serial::println("Rust mahiwa | check_1s");
    let start = arduino::micros();
    arduino::delay(1000);
    let end = arduino::micros();
    serial::println("delay 1 second");
    serial::print("time(micro): ");
    serial::print_i64(end - start);
    serial::println("");
}
