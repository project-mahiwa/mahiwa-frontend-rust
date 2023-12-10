#[link(wasm_import_module = "arduino")]
extern "C" {
    /*
     * Random Numbers
     */

    /*
     * Characters
     */

    /*
     * Trigonometry
     */

    /*
     * Math
     */

    /*
     * Analog I/O
     */
    #[link_name = "analogRead"]
    fn _analogRead(pin: u8) -> u16;
    #[link_name = "analogWrite"]
    fn _analogWrite(pin: u8, value: u8);

    /*
     * Digital I/O
     */
    #[link_name = "digitalRead"]
    fn _digitalRead(pin: u8) -> i32;
    #[link_name = "digitalWrite"]
    fn _digitalWrite(pin: u8, value: u8);
    #[link_name = "pinMode"]
    fn _pinMode(pin: u8, mode: u8);

    /*
     * Time
     */
    #[link_name = "delay"]
    fn _delay(ms: u32);
    #[link_name = "delayMicroseconds"]
    fn _delayMicroseconds(us: u32);
    #[link_name = "millis"]
    fn _millis() -> u64;
    #[link_name = "micros"]
    fn _micros() -> u64;
}

/*
* Random Numbers
*/

/*
 * Characters
*/

/*
 * Trigonometry
*/

/*
 * Math
*/

/*
 * Analog I/O
*/
/// Equivalent to Arduino's [analogRead](https://www.arduino.cc/reference/en/language/functions/analog-io/analogread/)
///
/// # Examples
///
/// ```
/// let read=arduino::analog_read(2);
/// ```
pub fn analog_read(pin: u8) -> u16 {
    unsafe { _analogRead(pin) }
}

/// Equivalent to Arduino's [analogWrite](https://www.arduino.cc/reference/en/language/functions/analog-io/analogwrite/)
///
/// value 0~255
///
/// # Examples
///
/// ```
/// arduino::analog_write(3,128);
/// ```
pub fn analog_write(pin: u8, value: u8) {
    unsafe {
        _analogWrite(pin, value);
    }
}


/*
 * Digital I/O
*/
/// Equivalent to Arduino's [digitalRead](https://www.arduino.cc/reference/en/language/functions/digital-io/digitalread/)
///
/// # Examples
///
/// ```
/// let read=arduino::digital_read(2);
/// ```
pub fn digital_read(pin: u8) -> i32 {
    unsafe { _digitalRead(pin) }
}

/// Equivalent to Arduino's [digitalWrite](https://www.arduino.cc/reference/en/language/functions/digital-io/digitalwrite/)
///
/// value 0:LOW, 1:HIGH
///
/// # Examples
///
/// ```
/// arduino::digital_write(3,1);
/// ```
pub fn digital_write(pin: u8, value: u8) {
    unsafe {
        _digitalWrite(pin, value);
    }
}

/// Equivalent to Arduino's [pinMode](https://www.arduino.cc/reference/en/language/functions/digital-io/pinmode/)
///
/// @todo ここのmodeの設定うまいことしたい
///
/// 1:INPUT, 2:OUTPUT, 3:INPUT_PULLUP
///
/// # Examples
///
/// ```
/// arduino::pin_mode(2,1);
/// ```
pub fn pin_mode(pin: u8, mode: u8) {
    unsafe {
        _pinMode(pin, mode);
    }
}

/*
* Time
*/
/// Equivalent to Arduino's [delay](https://www.arduino.cc/reference/en/language/functions/time/delay/)
///
/// # Examples
///
/// ```
/// arduino::delay(1000);
/// ```
pub fn delay(ms: u32) {
    unsafe {
        _delay(ms);
    }
}

/// Equivalent to Arduino's [delayMicroseconds](https://www.arduino.cc/reference/en/language/functions/time/delaymicroseconds/)
///
/// # Examples
///
/// ```
/// arduino::delay_microseconds(1000);
/// ```
pub fn delay_microseconds(us: u32) {
    unsafe {
        _delayMicroseconds(us);
    }
}
/// Equivalent to Arduino's [millis](https://www.arduino.cc/reference/en/language/functions/time/millis/)
///
/// # Examples
///
/// ```
/// let ms = arduino::millis();
/// ```
pub fn millis() -> u64 {
    unsafe { _millis() }
}
/// Equivalent to Arduino's [micros](https://www.arduino.cc/reference/en/language/functions/time/micros/)
///
/// # Examples
///
/// ```
/// let us = arduino::micros();
/// ```
pub fn micros() -> u64 {
    unsafe { _micros() }
}
