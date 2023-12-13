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
    serial::println("===SEED:1 ===");
    arduino::random_seed(1);
    serial::println(arduino::random().numtoa_str(10, &mut buf));
    serial::println(arduino::random().numtoa_str(10, &mut buf));
    arduino::random_seed(2);
    serial::println("===SEED:2 ===");
    serial::println(arduino::random().numtoa_str(10, &mut buf));
    serial::println(arduino::random().numtoa_str(10, &mut buf));
    serial::println("===SEED:1 ===");
    arduino::random_seed(1);
    serial::println(arduino::random().numtoa_str(10, &mut buf));
    serial::println(arduino::random().numtoa_str(10, &mut buf));

    // resut
    // ===SEED:1 ===
    // 1481765933
    // 1085377743
    // ===SEED:2 ===
    // 816048218
    // 688989553
    // ===SEED:1 ===
    // 1481765933
    // 1085377743

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
