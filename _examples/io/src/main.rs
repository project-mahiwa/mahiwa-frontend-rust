#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::println("Rust IO test");
    arduino::pin_mode(3, arduino::OUTPUT);
    loop {
        arduino::digital_write(3, arduino::HIGH);
        serial::println("change");
        arduino::delay(1000);
        arduino::digital_write(3, arduino::LOW);
        serial::println("change");
        arduino::delay(1000);
    }
}
