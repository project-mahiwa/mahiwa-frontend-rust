#![no_std]
#![no_main]
// mahiwa-backendが認識するWebAssemblyのエントリー関数が_startなのでそれに合わせる
use mahiwa_frontend_rust::arduino;
use mahiwa_frontend_rust::network;
use mahiwa_frontend_rust::serial;

#[no_mangle]
fn _start() {
    serial::println("Rust WiFi with Mahiwa");
    network::wlan_mode(network::WIFI_STA);
    network::wlan_connect("SSID", "PASSWORD");

    serial::println("connecting...");
    while !network::wlan_is_connected() {
        arduino::delay(1000);
        serial::print("Status Number : ");
        serial::print_i32(network::wlan_status());
        serial::println("");
    }
    serial::println("connected!");

    let local_ip = network::wlan_local_ip();
    serial::print("My Local Ip : ");
    serial::print(local_ip);
    serial::println("");
}
