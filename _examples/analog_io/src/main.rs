#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::println("Lang : Go | Task : analog io");
    loop {
		serial::print("Light sensor: ")
		serial::print_int(arduino::analog_read(4))
		serial::println("")
		arduino::delay(500)
    }
}
