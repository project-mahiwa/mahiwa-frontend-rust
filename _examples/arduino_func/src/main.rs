#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;
use core::ptr::write;  
use core::fmt::write;  
fn convert_i64_to_str(num: i64, buf: &mut [u8]) -> &str {
    let len = write(buf, num).unwrap();
    core::str::from_utf8(&buf[..len]).unwrap()
}


#[no_mangle]
fn _start() {
    // Arduino関数の動作確認
/*
 * Random Numbers
 */
// randomSeedが効いていることを確認する
    let mut buffer = [0u8; 20];
arduino::random_seed(1);
let rand1 = arduino::random();
serial::println( convert_i64_to_str(rand1, &mut buffer));
arduino::random_seed(2);
// serial::println();
arduino::random_seed(1);
// serial::println(arduino::random().to_str());

/*
 * Characters
 */
    //なし
    /*
     * Trigonometry
     */

    /*
     * Math
     */
    /*
     * analog I/O
     */
    /*
     * Digital I/O
     */
    /*
     * Time
     */
    serial::print("hello mahiwa written in Rust");
    loop {
        arduino::delay(900);
        serial::println("hello mahiwa");
    }
}
