#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

use numtoa::NumToA;

#[no_mangle]
fn _start() {
    let mut buf = [0u8; 20];
    // Arduino関数の動作確認
    /*
     * Random Numbers
     */
    // randomSeedが効いていることを確認する
    serial::println("😀");
    arduino::random_seed(1);
    let rand1 = arduino::random();
    serial::println(rand1.numtoa_str(10, &mut buf));
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
