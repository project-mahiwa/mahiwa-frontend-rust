#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    // Arduino関数の動作確認
    /*
     * Random Numbers
     */
    // randomSeedが効いていることを確認する
    serial::println("===SEED:1 ===");
    arduino::random_seed(1);
    serial::print_long(arduino::random());
    serial::println("");
    serial::print_long(arduino::random());
    serial::println("");
    arduino::random_seed(2);
    serial::println("===SEED:2 ===");
    serial::print_long(arduino::random());
    serial::println("");
    serial::print_long(arduino::random());
    serial::println("");
    serial::println("===SEED:1 ===");
    arduino::random_seed(1);
    serial::print_long(arduino::random());
    serial::println("");
    serial::print_long(arduino::random());
    serial::println("");

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
    serial::print("cos(0.0): ");
    let cos1 = arduino::cos(0.0);
    serial::print_double(cos1, 6);
    serial::println("");
    serial::print("cos(1.0): ");
    let cos2 = arduino::cos(1.0);
    serial::print_double(cos2, 6);
    serial::println("");

    serial::print("sin(1.0): ");
    serial::print_double(arduino::sin(1.0), 6);
    serial::println("");
    serial::print("sin(0.0): ");
    serial::print_double(arduino::sin(0.0), 6);
    serial::println("");

    serial::print("tan(1.0): ");
    serial::print_double(arduino::tan(1.0), 6);
    serial::println("");
    serial::print("tan(1.1): ");
    serial::print_double(arduino::tan(1.1), 6);
    serial::println("");

    // 動作結果

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
