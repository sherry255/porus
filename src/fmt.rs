//! Utilities for formatting and printing strings, i.e. alternative to
//! `std::fmt`
//!
//! This module contains the runtime support for the
//! [`f!`](../macro.f.html) macro. This macro uses a procedural macro
//! to emit calls to this module in order to format arguments at
//! runtime into strings.
//!
//! # Usage
//!
//! Since the procedural macro uses rustc's `libfmt_macros` to parse
//! the format string, thus the usage of [`f!`](../macro.f.html) macro
//! is very close to that of `std::format_args!`.
//!
//! ## Positional parameters
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"2 1 1 2", stringf!("{1:d} {:d} {0:d} {:d}", 1, 2).as_ref());
//! ```
//!
//! ## Formatting Types
//!
//! ### Character
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"A", stringf!("{:c}", 0x41).as_ref());
//! ```
//!
//! ### String
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"hello", stringf!("{:s}", "hello").as_ref());
//! ```
//!
//! ### Integer
//!
//! #### Decimal
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"123", stringf!("{:d}", 123).as_ref());
//! ```
//!
//! ### Floating-point number
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"0.125", stringf!("{:.3f}", 0.125).as_ref());
//! ```
//!
//! precision specified by a parameter
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"0.125", stringf!("{:.*f}", 3, 0.125).as_ref());
//! ```
//!
//! ### Format function
//!
//! ```
//! # use porus::prelude::*;
//! assert_eq!(b"hello", stringf!("{}", f!("hello")).as_ref());
//! ```

use crate::io::Sink;
use core::convert::TryInto;
use core::intrinsics::powif64;
use core::iter::Iterator;
use core::ops::{Div, Neg, Rem};
#[allow(unused_imports)]
use porus_macros::format;

/// The core macro for formatted string creation & output,
/// i.e. alternative to `std::format_args!`
///
/// Values returned by this macro can be passed to
/// [`stringf!`](macro.stringf.html),
/// [`writef!`](macro.writef.html),
/// [`writelnf!`](macro.writelnf.html).
///
/// ```
/// # use porus::prelude::*;
/// let s = stringf!("{}", f!("Hello, world!"));
/// assert_eq!(b"Hello, world!", s.as_ref());
/// ```
///
/// For more information, see the documentation in [`porus::fmt`](crate::fmt).
pub macro f($($arg:tt)*) {
    format!($($arg)*)
}

pub fn fwrite<S: Sink, F: FnMut(&mut S)>(sink: &mut S, mut f: F) {
    f(sink)
}

pub fn join<S: Sink, Sep: FnMut(&mut S), F: FnMut(&mut S), I: Iterator<Item = F>>(
    mut sep: Sep,
    mut it: I,
) -> impl FnMut(&mut S) {
    move |s: &mut S| {
        let iter = &mut it;

        match Iterator::next(iter) {
            None => {
                return;
            }
            Some(mut f) => {
                f(s);
            }
        }

        for mut f in iter {
            sep(s);
            f(s);
        }
    }
}

pub fn fwrite_str<S: Sink, T: AsRef<[u8]>>(s: &mut S, t: T) {
    for c in AsRef::<[u8]>::as_ref(&t) {
        Sink::write(s, *c);
    }
}

pub trait String {
    fn write<S: Sink>(self, s: &mut S);
}

impl<'a> String for &'a str {
    fn write<S: Sink>(self, s: &mut S) {
        fwrite_str(s, self);
    }
}

pub trait Int {
    fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize);
}

fn to_char(d: u8) -> u8 {
    match d {
        0...9 => b'0' + d,
        10...35 => b'A' + d - 10,
        _ => panic!(),
    }
}

fn write_unsigned<
    S: Sink,
    T: Copy + Default + PartialOrd + Div<Output = T> + Rem<Output = T> + TryInto<u8>,
>(
    s: &mut S,
    mut x: T,
    radix: T,
    width: usize,
) {
    let mut buf = [b'0'; 40];
    let mut i = 39;

    while x > Default::default() {
        buf[i] = to_char(TryInto::try_into(x % radix).ok().unwrap());
        i -= 1;
        x = x / radix;
    }

    i = Ord::min(i + 1, 40 - width);
    fwrite_str(s, &buf[i..]);
}

fn write_signed<
    S: Sink,
    T: Copy + Default + PartialOrd + Neg<Output = T> + Div<Output = T> + Rem<Output = T> + TryInto<u8>,
>(
    s: &mut S,
    x: T,
    radix: T,
    width: usize,
) {
    if x < -x {
        Sink::write(s, b'-');
        write_unsigned(s, -x, radix, width);
    } else {
        write_unsigned(s, x, radix, width);
    }
}

#[doc(hidden)]
macro unsigned($t:ty) {
    impl Int for $t {
        fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
            write_unsigned(s, self, From::from(radix), width)
        }
    }

    impl<'a> Int for &'a $t {
        fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
            Int::write(*self, s, radix, width)
        }
    }
}

#[doc(hidden)]
macro signed($t:ty) {
    impl Int for $t {
        fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
            write_signed(s, self, From::from(radix), width)
        }
    }

    impl<'a> Int for &'a $t {
        fn write<S: Sink>(self, s: &mut S, radix: u8, width: usize) {
            Int::write(*self, s, radix, width)
        }
    }
}

unsigned!(u8);
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(u128);
unsigned!(usize);

// signed!(i8);
signed!(i16);
signed!(i32);
signed!(i64);
signed!(i128);
signed!(isize);

pub trait Float {
    fn write<S: Sink>(self, s: &mut S, prec: usize);
}

impl Float for f64 {
    fn write<S: Sink>(mut self, s: &mut S, prec: usize) {
        if self.is_finite() {
            #[cfg(feature = "local-judge")]
            {
                fwrite_str(s, b"\x1bXf.");
                write_unsigned(s, prec, 10, 1);
                fwrite_str(s, b"\x1b\\");
            }

            if self.is_sign_negative() {
                Sink::write(s, b'-');
                self = -self;
            }

            self *= unsafe { powif64(10.0, TryInto::try_into(prec).ok().unwrap()) };
            let m = 10_u64.pow(TryInto::try_into(prec).ok().unwrap());

            if self <= 9_007_199_254_740_992.0 {
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                let i = self as u64;
                write_unsigned(s, i / m, 10, 1);
                Sink::write(s, b'.');
                write_unsigned(s, i % m, 10, prec);
                return;
            }
        }

        panic!("floating number out of range");
    }
}

impl<'a> Float for &'a f64 {
    fn write<S: Sink>(self, s: &mut S, prec: usize) {
        Float::write(*self, s, prec)
    }
}
