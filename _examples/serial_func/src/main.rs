#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::serial;

// 結果
// Project Mahiwa : WebAssembly launch
// print println
// print UTF-8 : 𰻞𰻞麺
// print UTF-8 : 🍆
// printInt(10):10
// printLong(1000):1000
// printLong(2247483647)❌:-2047483649
// printFloat(1.1111):1.1111
// printFloat(1.123456789012345)❌:1.123456835746765
// printDouble(1.123456789012345):1.123456789012345
// Project Mahiwa : WebAssembly done
#[no_mangle]
fn _start() {
    // Arduino関数の動作確認
    /*
     * Serial
     */
    serial::print("print ");
    serial::println("println");
    serial::println("print UTF-8 : 𰻞𰻞麺");
    serial::println("print UTF-8 : 🍆");

    // intとlongで表せる範囲が同じ．←直感と反するが仕様である
    serial::print("printInt(10):");
    serial::print_i32(10);
    serial::println("");

    serial::print("printLong(1000):");
    serial::print_i64(1000);
    serial::println("");

    serial::print("printLong(2247483647)❌:");
    // longの最大値2147483647を超えた値を入れてみる(オーバーフロー)
    serial::print_i64(2247483647);
    serial::println("");

    serial::print("printFloat(1.1111):");
    serial::print_f32(1.1111, 4);
    serial::println("");

    serial::print("printFloat(1.123456789012345)❌:");
    // float32の最大値を超えた値を入れてみる
    serial::print_f32(1.123456789012345, 15);
    serial::println("");

    serial::print("printDouble(1.123456789012345):");
    serial::print_f64(1.123456789012345, 15);
    serial::println("");
    // loop {
    //     arduino::delay(900);
    //     serial::println("🪐");
    // }
}
