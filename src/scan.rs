use crate::io::{PeekableSource, Source};
use crate::math::powi;
use crate::utils::unwrap;
use core::convert::From;
use core::ops::{Add, Mul, Neg};

pub trait Consumer {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool;
}

pub fn fread<I: Source, C: Consumer>(s: &mut PeekableSource<I>, c: C) -> bool {
    Consumer::consume(c, s)
}

pub fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' | b'\t'..=b'\r' => true,
        _ => false,
    }
}

pub struct Whitespace;

impl Consumer for Whitespace {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        while let Some(&c) = s.peek() {
            if is_whitespace(c) {
                s.consume();
            } else {
                break;
            }
        }
        true
    }
}

pub struct Char<'a>(pub &'a mut u8);

impl<'a> Consumer for Char<'a> {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        match s.peek() {
            None => false,
            Some(&c) => {
                *(self.0) = c;
                s.consume();
                true
            }
        }
    }
}

pub struct Int<'a, T: 'a>(&'a mut T, u8);

pub fn bin<'a, T: 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 2)
}

pub fn oct<'a, T: 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 8)
}

pub fn hex<'a, T: 'a>(i: &'a mut T) -> Int<'a, T> {
    Int(i, 16)
}

fn read_digit<I: Source>(s: &mut PeekableSource<I>, radix: u8) -> Option<u8> {
    let c = match s.peek() {
        None => {
            return None;
        }
        Some(&x) => x,
    };

    let d = match c {
        b'0'..=b'9' => u8::wrapping_sub(c, b'0'),
        b'A'..=b'Z' => u8::wrapping_sub(c, b'7'),
        b'a'..=b'z' => u8::wrapping_sub(c, b'W'),
        _ => {
            return None;
        }
    };

    if d >= radix {
        return None;
    }

    s.consume();
    Some(d)
}

fn read_unsigned<I: Source, T: Copy + Default + Add<Output = T> + Mul<Output = T> + From<u8>>(
    s: &mut PeekableSource<I>,
    radix: u8,
) -> Option<T> {
    match read_digit(s, radix) {
        None => None,
        Some(d) => {
            let mut x: T = From::from(d);

            while let Some(d) = read_digit(s, radix) {
                x = x * From::from(10) + From::from(d);
            }

            Some(x)
        }
    }
}

fn read_signed<
    I: Source,
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + From<u8> + Neg<Output = T>,
>(
    s: &mut PeekableSource<I>,
    radix: u8,
) -> Option<T> {
    match s.peek() {
        None => None,
        Some(&b'-') => {
            s.consume();
            let r: Option<T> = read_unsigned(s, radix);
            r.map(|x| -x)
        }
        Some(_) => read_unsigned(s, radix),
    }
}

impl<'a, T: 'a + Copy + Default + Add<Output = T> + Mul<Output = T> + From<u8>> Consumer
    for Int<'a, T>
{
    default fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        match read_unsigned(s, self.1) {
            None => false,
            Some(v) => {
                *self.0 = v;
                true
            }
        }
    }
}

impl<
        'a,
        T: 'a + Copy + Default + Add<Output = T> + Mul<Output = T> + From<u8> + Neg<Output = T>,
    > Consumer for Int<'a, T>
{
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        match read_signed(s, self.1) {
            None => false,
            Some(v) => {
                *self.0 = v;
                true
            }
        }
    }
}

#[doc(hidden)]
macro int($t:ty) {
    impl<'a> Consumer for &'a mut $t {
        fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
            Consumer::consume(Int(self, 10), s)
        }
    }
}

int!(u8);
int!(u16);
int!(u32);
int!(u64);
int!(u128);
int!(usize);

// int!(i8);
int!(i16);
int!(i32);
int!(i64);
int!(i128);
int!(isize);

#[allow(clippy::float_arithmetic)]
impl<'a> Consumer for &'a mut f64 {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        let sign: f64 = if let Some(&b'-') = s.peek() {
            s.consume();
            -1.0
        } else {
            1.0
        };

        let mut int: u64 = 0;
        fread(s, &mut int);

        let mut exp: i32 = 0;

        if let Some(&b'.') = s.peek() {
            s.consume();

            while let Some(d) = read_digit(s, 10) {
                int = int * 10 + u64::from(d);
                exp = unwrap(i32::checked_sub(exp, 1));
            }
        }

        if let Some(&b'e') = s.peek() {
            s.consume();
            let mut e: i32 = 0;
            fread(s, &mut e);
            exp = unwrap(i32::checked_add(exp, e));
        }

        #[allow(clippy::cast_precision_loss)]
        {
            *self = sign * powi(10.0, exp) * (int as f64);
        }

        true
    }
}

impl<'a> Consumer for &'a mut [u8] {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        for elem in self.iter_mut() {
            match s.peek() {
                None => return false,
                Some(&c) => {
                    *elem = c;
                    s.consume();
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{fread, hex, Whitespace};

    #[test]
    fn test_whitespace() {
        let source = &mut From::from(b"   " as &_);
        fread(source, Whitespace);
        assert!(source.eof());
    }

    #[test]
    fn test_unsigned_match() {
        let source = &mut From::from(b"a" as &_);
        let mut x = 0_usize;
        fread(source, hex(&mut x));
        assert!(x == 0xa);
    }

    #[test]
    fn test_unsigned_mismatch() {
        let source = &mut From::from(b"g" as &_);
        let mut x = 0_usize;
        assert!(!fread(source, hex(&mut x)));
    }

    #[test]
    fn test_unsigned_mismatch_empty() {
        let source = &mut From::from(b"" as &_);
        let mut x = 0_usize;
        assert!(!fread(source, hex(&mut x)));
    }

    #[test]
    fn test_signed_match() {
        let source = &mut From::from(b"-123" as &_);
        let mut x = 0_isize;
        fread(source, &mut x);
        assert!(x == -123);
    }

    #[test]
    fn test_signed_mismatch() {
        let source = &mut From::from(b"-g" as &_);
        let mut x = 0_isize;
        assert!(!fread(source, &mut x));
    }

    #[test]
    fn test_signed_mismatch_empty() {
        let source = &mut From::from(b"" as &_);
        let mut x = 0_isize;
        assert!(!fread(source, &mut x));
    }

    #[test]
    fn test_signed_mismatch_sign() {
        let source = &mut From::from(b"-" as &_);
        let mut x = 0_isize;
        assert!(!fread(source, &mut x));
    }
}
