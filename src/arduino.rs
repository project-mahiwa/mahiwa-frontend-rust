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

    /*
     * Digital I/O
    */

    /*
    * Time
    */
    #[link_name = "delay"]
    fn _delay(ms: u32);
    #[link_name = "delayMicroseconds"]
    fn _delayMicroseconds(us: u32);
    #[link_name = "millis"]
    fn _millis()->u64;
    #[link_name = "micros"]
    fn _micros()->u64;
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

/*
 * Digital I/O
*/

/*
* Time
*/
/// Equivalent to Arduino's delay
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

/// Equivalent to Arduino's delayMicroseconds
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
/// Equivalent to Arduino's millis
///
/// # Examples
///
/// ```
/// let time = arduino::millis();
/// ```
pub fn millis()->u64 {
    unsafe {
        _millis()
    }
}
/// Equivalent to Arduino's millis
///
/// # Examples
///
/// ```
/// let time = arduino::millis();
/// ```
pub fn micros()->u64 {
    unsafe {
        _micros()
    }
}
