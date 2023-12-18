#[link(wasm_import_module = "arduino")]
extern "C" {
    /*
     * Random Numbers
     */
    #[link_name = "randomArduino"]
    fn _random() -> i64;
    #[link_name = "randomSeed"]
    fn _randomSeed(seed: i64);

    /*
     * Characters
     */
    // 一旦実装を保留する

    /*
     * Trigonometry
     */
    #[link_name = "cos"]
    fn _cos(rad: f64) -> f64;
    #[link_name = "sin"]
    fn _sin(rad: f64) -> f64;
    #[link_name = "tan"]
    fn _tan(rad: f64) -> f64;

    /*
     * Math
     */
    #[link_name = "abs"]
    fn _abs(x: i32) -> i32;
    #[link_name = "constrain"]
    fn _constrain(x: i32, a: i32, b: i32) -> i32;
    #[link_name = "map"]
    fn _map(value: i64, from_low: i64, from_high: i64, to_low: i64, to_high: i64) -> i64;
    #[link_name = "max"]
    fn _max(x: i64, y: i64) -> i64;
    #[link_name = "min"]
    fn _min(x: i64, y: i64) -> i64;
    #[link_name = "powArduino"]
    fn _pow(base: i64, exponent: i64) -> f64;
    #[link_name = "sq"]
    fn _sq(x: i32) -> i32;
    #[link_name = "sqrtArduino"]
    fn _sqrt(x: i32) -> f64;

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

// pinMode
pub const INPUT: u8 = 1;
pub const OUTPUT: u8 = 2;
pub const INPUT_PULLUP: u8 = 3;
// digitalWrite
pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;

/*
* Random Numbers
*/
//// Equivalent to Arduino's [random](https://www.arduino.cc/reference/en/language/functions/random-numbers/random/)
///
/// # Examples
/// ```
/// let result = arduino::random(); //result=0~9
/// ```
pub fn random() -> i64 {
    unsafe { _random() }
}

//// Equivalent to Arduino's [randomSeed](https://www.arduino.cc/reference/en/language/functions/random-numbers/randomseed/)
///
/// # Examples
/// ```
/// arduino::random_seed(0);
/// ```
pub fn random_seed(seed: i64) {
    unsafe {
        _randomSeed(seed);
    }
}

/*
 * Characters
*/

/*
 * Trigonometry
*/
/// Equivalent to Arduino's [cos](https://www.arduino.cc/reference/en/language/functions/trigonometry/cos/)
///
/// # Examples
/// ```
/// let result = arduino::cos(0.0); //result=1.0
/// ```
pub fn cos(rad: f64) -> f64 {
    unsafe { _cos(rad) }
}

/// Equivalent to Arduino's [sin](https://www.arduino.cc/reference/en/language/functions/trigonometry/sin/)
///
/// # Examples
/// ```
/// let result = arduino::sin(0.0); //result=0.0
/// ```
pub fn sin(rad: f64) -> f64 {
    unsafe { _sin(rad) }
}

/// Equivalent to Arduino's [tan](https://www.arduino.cc/reference/en/language/functions/trigonometry/tan/)
///
/// # Examples
/// ```
/// let result = arduino::tan(0.0); //result=1.0
/// ```
pub fn tan(rad: f64) -> f64 {
    unsafe { _tan(rad) }
}

/*
 * Math
*/
/// Equivalent to Arduino's [abs](https://www.arduino.cc/reference/en/language/functions/math/abs/)
///
/// # Examples
///
/// ```
/// let result = arduino::abs(-2); //2
/// ```
pub fn abs(x: i32) -> i32 {
    unsafe { _abs(x) }
}

/// Equivalent to Arduino's [constrain](https://www.arduino.cc/reference/en/language/functions/math/constrain/)
///
/// # Examples
///
/// ```
/// // 10~100の範囲に値を制限する(sensor_valueが150なら100,0なら10,11なら11になる)
/// let result = arduino::constrain(sensor_value,10,100);
/// ```
pub fn constrain(x: i32, a: i32, b: i32) -> i32 {
    unsafe { _constrain(x, a, b) }
}

/// Equivalent to Arduino's [map](https://www.arduino.cc/reference/en/language/functions/math/map/)
///
/// # Examples
///
/// ```
///  // 10bitの値を8bitにまるめる
///  let result = arduino::map(sensor_value,0,1023,0,255);
/// ```
pub fn map(value: i64, from_low: i64, from_high: i64, to_low: i64, to_high: i64) -> i64 {
    unsafe { _map(value, from_low, from_high, to_low, to_high) }
}

/// Equivalent to Arduino's [max](https://www.arduino.cc/reference/en/language/functions/math/max/)
///
/// # Examples
///
/// ```
///  let result = arduino::max(2,3);//result=3
/// ```
pub fn max(x: i64, y: i64) -> i64 {
    unsafe { _max(x, y) }
}

/// Equivalent to Arduino's [min](https://www.arduino.cc/reference/en/language/functions/math/min/)
///
/// # Examples
///
/// ```
///  let result = arduino::min(2,3);//result=2
/// ```
pub fn min(x: i64, y: i64) -> i64 {
    unsafe { _min(x, y) }
}

/// Equivalent to Arduino's [pow](https://www.arduino.cc/reference/en/language/functions/math/pow/)
///
/// # Examples
/// ```
/// let result = arduino::pow(2,3);//result=8
/// ```
pub fn pow(base: i64, exponent: i64) -> f64 {
    unsafe { _pow(base, exponent) }
}

/// Equivalent to Arduino's [sq](https://www.arduino.cc/reference/en/language/functions/math/sq/)
///
/// # Examples
/// ```
/// let result = arduino::sq(2); //result=4
/// ```
pub fn sq(x: i32) -> i32 {
    unsafe { _sq(x) }
}

/// Equivalent to Arduino's [sqrt](https://www.arduino.cc/reference/en/language/functions/math/sqrt/)
///
/// # Examples
/// ```
/// let result = arduino::sqrt(4); //result=2.0
/// ```
pub fn sqrt(x: i32) -> f64 {
    unsafe { _sqrt(x) }
}

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
