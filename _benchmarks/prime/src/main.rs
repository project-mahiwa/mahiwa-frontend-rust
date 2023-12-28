#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::{arduino, serial};

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    let num_max = (num as f64).sqrt() as i32;
    for i in 2..=num_max {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

#[no_mangle]
fn _start() {
    arduino::delay(5000);
    serial::println("Rust mahiwa | prime");
    let start = arduino::micros();
    let result = is_prime(2147483647);
    let end = arduino::micros();
    serial::print("result: ");
    if result {
        serial::print("prime");
    } else {
        serial::print("not prime");
    }
    serial::println("");
    serial::print("time(micro): ");
    serial::print_i64(end - start);
    serial::println("");
}
