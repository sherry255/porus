#[macro_use]
extern crate porus;
use porus::prelude::*;

#[no_mangle]
pub extern "C" fn divide(dividend: i32, divisor: i32) -> i32 {
    match dividend.overflowing_div(divisor) {
        (x, false) => x,
        (_, true) => i32::max_value(),
    }
}
