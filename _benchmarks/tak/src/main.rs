#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::{arduino, serial};

fn tak(x: i32, y: i32, z: i32) -> i32 {
    if y < x {
        tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y))
    } else {
        z
    }
}

#[no_mangle]
fn _start() {
    arduino::delay(5000);
    serial::println("Rust mahiwa | check_1s");
    let start = arduino::micros();
    let result = tak(14, 5, 0);
    let end = arduino::micros();
    serial::print("result: ");
    serial::print_i32(result);
    serial::println("");
    serial::print("time(micro): ");
    serial::print_i64(end - start);
    serial::println("");
}
