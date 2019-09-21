#[allow(unused_imports)]
use crate::fmt::{self, fwrite_str};
#[allow(unused_imports)]
use crate::fmt::{f, fwrite};
use crate::io::Sink;
use alloc::alloc::{Alloc, Global};
use core::cmp::Ordering;
use core::mem::size_of;
use core::ops::Deref;
use core::ptr::NonNull;
use core::slice::from_raw_parts;
use core::str;

mod buffer;
pub use self::buffer::Buffer as StringBuffer;

#[derive(Clone, Copy)]
struct SharedString {
    counter: NonNull<usize>,
    length: usize,
    s: NonNull<u8>,
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
            Tag::Shared => self.shared.s.as_ptr(),
            Tag::Inline => self.inline.s.as_ptr(),
            Tag::Static => self.static_.s,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len() as usize) }
    }
}

impl Clone for Union {
    fn clone(&self) -> Self {
        unsafe {
            match self.tag() {
                Tag::Shared => {
                    *self.shared.counter.as_ptr() += 1;
                    Self {
                        shared: Clone::clone(&self.shared),
                    }
                }
                Tag::Inline => Self {
                    inline: Clone::clone(&self.inline),
                },
                Tag::Static => Self {
                    static_: Clone::clone(&self.static_),
                },
            }
        }
    }
}

pub struct String<A: Alloc = Global> {
    s: Union,
    allocator: A,
}

impl<A: Alloc + Default> From<&'static [u8]> for String<A> {
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

impl<A: Alloc> Deref for String<A> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_ref()) }
    }
}

impl<A: Alloc> AsRef<[u8]> for String<A> {
    fn as_ref(&self) -> &[u8] {
        self.s.as_bytes()
    }
}

impl<A: Alloc> PartialEq for String<A> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_ref(), other.as_ref())
    }
}

impl<A: Alloc> PartialOrd for String<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_ref(), other.as_ref())
    }
}

impl<A: Alloc + Clone> Clone for String<A> {
    fn clone(&self) -> Self {
        Self {
            s: Clone::clone(&self.s),
            allocator: Clone::clone(&self.allocator),
        }
    }
}

impl<A: Alloc> Drop for String<A> {
    fn drop(&mut self) {
        if let Tag::Shared = self.s.tag() {
            let counter_size = size_of::<usize>();
            unsafe {
                *self.s.shared.counter.as_mut() -= 1;
                if *self.s.shared.counter.as_ref() == 0 {
                    Alloc::dealloc_array::<u8>(
                        &mut self.allocator,
                        NonNull::cast(self.s.shared.counter),
                        counter_size + self.s.shared.length,
                    )
                    .expect("dealloc failed");
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
        let mut buffer = <StringBuffer as Default>::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abc" as &'static [u8]);
        assert!(s1 == s2);
    }

    #[test]
    fn test_shared_string_buffer() {
        let source = &mut From::from(b"abcdefghijklmnopqrstuvwxyz" as &_);
        let mut buffer = <StringBuffer as Default>::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abcdefghijklmnopqrstuvwxyz" as &'static [u8]);
        assert!(s1 == s2);
    }
}
