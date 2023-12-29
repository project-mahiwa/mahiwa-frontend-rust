use std::ffi::CStr;
use std::os::raw::c_char;

#[link(wasm_import_module = "network")]
extern "C" {
    #[link_name = "wlanMode"]
    fn _wlan_mode(mode: i32);
    #[link_name = "wlanConnect"]
    fn _wlan_connect(ssid: *const u8, ssid_len: usize, password: *const u8, password_len: usize);
    #[link_name = "wlanStatus"]
    fn _wlan_status() -> i32;
    #[link_name = "wlanLocalIp"]
    fn _wlan_local_ip(buf: *mut i8);
}

// wifi mode
pub const WIFI_OFF: i32 = 0;
pub const WIFI_STA: i32 = 1;
pub const WIFI_AP: i32 = 2;
pub const WIFI_AP_STA: i32 = 3;

// wifi status
pub const WL_NO_SHIELD: i32 = 255;
pub const WL_IDLE_STATUS: i32 = 0;
pub const WL_NO_SSID_AVAIL: i32 = 1;
pub const WL_SCAN_COMPLETED: i32 = 2;
pub const WL_CONNECTED: i32 = 3;
pub const WL_CONNECT_FAILED: i32 = 4;
pub const WL_CONNECTION_LOST: i32 = 5;
pub const WL_DISCONNECTED: i32 = 6;

/// Equivalent to Arduino's wifi
///
/// # Examples
///
/// ```
/// network::wlan_mode(WIFI_STA);
/// ```
pub fn wlan_mode(mode: i32) {
    unsafe { _wlan_mode(mode) }
}

/// connect WiFi with SSID and PassWord
///
/// ```
/// network::wlan_connect("SSID","PASSWORD");
/// ```
pub fn wlan_connect(ssid: &str, password: &str) {
    unsafe { _wlan_connect(ssid.as_ptr(), ssid.len(), password.as_ptr(), password.len()) }
}

/// wifi status
///
/// ```
/// let status = network::wlan_status();
/// ```
pub fn wlan_status() -> i32 {
    unsafe { _wlan_status() }
}

/// wifi status
///
/// ```
/// if(network::wlan_is_connected()){}
/// ```
pub fn wlan_is_connected() -> bool {
    if wlan_status() == WL_CONNECTED {
        return true;
    }
    return false;
}

/// get local ip
///
/// ```
/// let local_ip = network::wlan_local_ip();
/// ```
pub fn wlan_local_ip() -> String {
    let mut buf = [0 as c_char; 16];
    unsafe {
        _wlan_local_ip(buf.as_mut_ptr());
        CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
    }
}
