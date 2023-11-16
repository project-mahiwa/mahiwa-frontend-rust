#![no_std]

use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

fn main() {
    serial::print("hello mahiwa written in Rust");
    loop {
        arduino::delay(900);
        serial::print("hello mahiwa");
    }
}
