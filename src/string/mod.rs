use core::slice::from_raw_parts;
use crate::allocator::{deallocate, Allocator};
#[allow(unused_imports)]
use crate::fmt::{self, fwrite_str};
#[allow(unused_imports)]
use crate::fmt::{f, fwrite};
use crate::io::Sink;
use crate::os;

mod buffer;
pub use self::buffer::Buffer as StringBuffer;

#[derive(Clone, Copy)]
struct SharedString {
    counter: *mut usize,
    length: usize,
    s: *const u8,
}

#[cfg(all(target_endian = "big", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineString {
    s: [u8; 23],
    length: u8,
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineString {
    length: u8,
    s: [u8; 23],
}

#[cfg(all(target_endian = "big", target_pointer_width = "32"))]
struct InlineString {
    s: [u8; 11],
    length: u8,
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[derive(Clone, Copy)]
struct InlineString {
    length: u8,
    s: [u8; 11],
}

#[cfg(target_endian = "big")]
#[derive(Clone, Copy)]
struct StaticString {
    s: *const u8,
    length: usize,
    _padding: usize,
}

#[cfg(target_endian = "little")]
#[derive(Clone, Copy)]
struct StaticString {
    _padding: usize,
    length: usize,
    s: *const u8,
}

union Union {
    shared: SharedString,
    inline: InlineString,
    static_: StaticString,
}

enum Tag {
    Shared,
    Inline,
    Static,
}

impl Union {
    fn tag(&self) -> Tag {
        match unsafe { self.inline.length & 0x3 } {
            0 => Tag::Shared,
            1 => Tag::Inline,
            2 => Tag::Static,
            _ => unreachable!(),
        }
    }

    fn len(&self) -> usize {
        unsafe {
            match self.tag() {
                Tag::Shared => self.shared.length,
                Tag::Inline => (self.inline.length >> 2) as usize,
                Tag::Static => self.static_.length,
            }
        }
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        match self.tag() {
            Tag::Shared => self.shared.s,
            Tag::Inline => self.inline.s.as_ptr(),
            Tag::Static => self.static_.s,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len() as usize) }
    }
}

pub struct String<A: Allocator = os::Allocator> {
    s: Union,
    allocator: A,
}

impl<A: Allocator + Default> From<&'static [u8]> for String<A> {
    fn from(s: &'static [u8]) -> Self {
        Self {
            s: Union {
                static_: StaticString {
                    s: s.as_ptr(),
                    length: s.len(),
                    _padding: 2,
                },
            },
            allocator: Default::default(),
        }
    }
}

impl<A: Allocator> AsRef<[u8]> for String<A> {
    fn as_ref(&self) -> &[u8] {
        self.s.as_bytes()
    }
}

impl<A: Allocator> PartialEq for String<A> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_ref(), other.as_ref())
    }
}

impl<A: Allocator> Drop for String<A> {
    fn drop(&mut self) {
        if let Tag::Shared = self.s.tag() {
            unsafe {
                *self.s.shared.counter -= 1;
                if *self.s.shared.counter == 0 {
                    deallocate(&mut self.allocator, self.s.shared.counter);
                }
            }
        }
    }
}

impl<'a> fmt::String for &'a String {
    fn write<S: Sink>(self, s: &mut S) {
        fwrite_str(s, self);
    }
}

/// Create [`String`](string::String) using interpolation of runtime
/// expressions, i.e. alternative to `format!` in `std!`.
///
/// # Examples
/// ```
/// # use porus::prelude::*;
/// assert_eq!(b"test", stringf!("test").as_ref());
/// assert_eq!(b"hello world", stringf!("hello {:s}", "world").as_ref());
/// assert_eq!(b"x = 10, y = 30", stringf!("x = {:d}, y = {:d}", 10, 30).as_ref());
/// ```

pub macro stringf($($arg:tt)*) {
    {
        let mut buffer: StringBuffer = Default::default();
        fwrite(&mut buffer, &mut f!($($arg)*));
        let string: String = From::from(buffer);
        string
    }
}

#[cfg(test)]
mod tests {
    use super::{String, StringBuffer};
    use crate::scan::fread;

    #[test]
    fn test_inline_string_buffer() {
        let source = &mut From::from(b"abc " as &_);
        #[allow(clippy::default_trait_access)]
        let mut buffer: StringBuffer = Default::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abc" as &'static [u8]);
        assert!(s1 == s2);
    }

    #[test]
    fn test_shared_string_buffer() {
        let source = &mut From::from(b"abcdefghijklmnopqrstuvwxyz" as &_);
        #[allow(clippy::default_trait_access)]
        let mut buffer: StringBuffer = Default::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abcdefghijklmnopqrstuvwxyz" as &'static [u8]);
        assert!(s1 == s2);
    }
}
