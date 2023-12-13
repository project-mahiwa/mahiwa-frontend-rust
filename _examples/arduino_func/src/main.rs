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
    serial::print_int(arduino::random());
    serial::print_int(arduino::random());
    serial::println("");

    arduino::random_seed(2);
    serial::println("===SEED:2 ===");
    serial::print_int(arduino::random());
    serial::print_int(arduino::random());
    serial::println("===SEED:1 ===");
    arduino::random_seed(1);
    serial::print_int(arduino::random());
    serial::print_int(arduino::random());

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
    serial::print("cos(0): ");
    // rustには?:の３項演算子がない
    // 1.0ではない
    serial::print_double(arduino::cos(0));
    serial::print("sin(180): ");
    serial::print_double(arduino::sin(180));
    serial::print("tan(360): ");
    serial::print_double(arduino::tan(360));

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
