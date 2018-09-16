use core::marker::PhantomData;
use core::mem::{forget, size_of, transmute_copy};
use core::ptr::copy_nonoverlapping;
use core::slice::from_raw_parts;
use porus::alloc::{allocate, deallocate, reallocate, Allocator};
use porus::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use porus::io::read::{is_whitespace, Consumer};
use porus::io::{write, PeekableSource, Sink, Source};
use porus::os::OSAllocator;

#[cfg(all(target_endian = "little"))]
#[derive(Clone, Copy)]
struct SharedStringBuffer {
    s: *mut u8,
    capacity: usize,
    offset: usize,
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineStringBuffer {
    offset: u8,
    s: [u8; 23],
}

impl InlineStringBuffer {
    fn offset(&self) -> u8 {
        self.offset >> 2
    }
}

union StringBufferUnion {
    shared: SharedStringBuffer,
    inline: InlineStringBuffer,
}

impl StringBufferUnion {
    fn is_inline(&self) -> bool {
        match unsafe { self.inline.offset & 0x3 } {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
}

pub struct StringBuffer<P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = OSAllocator> {
    buffer: StringBufferUnion,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<P: CapacityPolicy, A: Allocator + Default> Default for StringBuffer<P, A> {
    fn default() -> Self {
        StringBuffer {
            buffer: StringBufferUnion {
                inline: InlineStringBuffer {
                    offset: 1,
                    s: [0; 23],
                },
            },
            allocator: Default::default(),
            _policy: PhantomData,
        }
    }
}

impl<P: CapacityPolicy, A: Allocator> Sink for StringBuffer<P, A> {
    fn write(&mut self, c: u8) {
        let counter_size = size_of::<usize>();

        if self.buffer.is_inline() {
            unsafe {
                let offset = self.buffer.inline.offset() as usize;
                let size = self.buffer.inline.s.len();
                if offset < size {
                    self.buffer.inline.s[offset] = c;
                    self.buffer.inline.offset = ((offset as u8 + 1) << 2) | 1;
                    return;
                } else {
                    let capacity = P::initial(size);
                    let s: *mut u8 = allocate(&mut self.allocator, counter_size + capacity);
                    copy_nonoverlapping(
                        self.buffer.inline.s.as_ptr(),
                        s.offset(counter_size as isize),
                        size,
                    );
                    self.buffer.shared = SharedStringBuffer {
                        s,
                        capacity,
                        offset,
                    };
                }
            }
        }

        unsafe {
            let offset = self.buffer.shared.offset;
            if offset == self.buffer.shared.capacity {
                let capacity = P::grow(self.buffer.shared.capacity);
                self.buffer.shared.capacity = capacity;
                self.buffer.shared.s = reallocate(
                    &mut self.allocator,
                    self.buffer.shared.s,
                    counter_size + capacity,
                );
            }
            *self
                .buffer
                .shared
                .s
                .offset((counter_size + offset) as isize) = c;
            self.buffer.shared.offset += 1;
        }
    }
}

impl<'a, P: CapacityPolicy, A: Allocator> Consumer for &'a mut StringBuffer<P, A> {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        while let Some(&c) = s.peek() {
            if !is_whitespace(c) {
                Sink::write(self, c);
                s.consume();
            } else {
                break;
            }
        }

        true
    }
}

impl<P: CapacityPolicy, A: Allocator> Drop for StringBuffer<P, A> {
    fn drop(&mut self) {
        if !self.buffer.is_inline() {
            unsafe {
                deallocate(&mut self.allocator, self.buffer.shared.s);
            }
        }
    }
}

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

union StringUnion {
    shared: SharedString,
    inline: InlineString,
    static_: StaticString,
}

enum Tag {
    Shared,
    Inline,
    Static,
}

use self::Tag::*;

impl StringUnion {
    fn tag(&self) -> Tag {
        match unsafe { self.inline.length & 0x3 } {
            0 => Shared,
            1 => Inline,
            2 => Static,
            _ => unreachable!(),
        }
    }

    fn len(&self) -> usize {
        unsafe {
            match self.tag() {
                Shared => self.shared.length,
                Inline => (self.inline.length >> 2) as usize,
                Static => self.static_.length,
            }
        }
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        match self.tag() {
            Shared => self.shared.s,
            Inline => self.inline.s.as_ptr(),
            Static => self.static_.s,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len() as usize) }
    }
}

pub struct String<A: Allocator = OSAllocator> {
    s: StringUnion,
    allocator: A,
}

struct StringBufferTransmute<P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = OSAllocator>
{
    buffer: StringBufferUnion,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<A: Allocator + Default> From<&'static [u8]> for String<A> {
    fn from(s: &'static [u8]) -> Self {
        String {
            s: StringUnion {
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

impl<P: CapacityPolicy, A: Allocator> From<StringBuffer<P, A>> for String<A> {
    fn from(x: StringBuffer<P, A>) -> Self {
        let mut buf: StringBufferTransmute<P, A> = unsafe { transmute_copy(&x) };
        forget(x);

        let counter_size = size_of::<usize>();

        if buf.buffer.is_inline() {
            unsafe {
                String {
                    s: StringUnion {
                        inline: InlineString {
                            length: buf.buffer.inline.offset,
                            s: buf.buffer.inline.s,
                        },
                    },
                    allocator: buf.allocator,
                }
            }
        } else {
            unsafe {
                let length = buf.buffer.shared.offset;
                let s = reallocate(
                    &mut buf.allocator,
                    buf.buffer.shared.s,
                    counter_size + length,
                );
                #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
                let counter = s as *mut usize;
                *counter = 1;

                String {
                    s: StringUnion {
                        shared: SharedString {
                            counter,
                            length,
                            s: s.offset(counter_size as isize),
                        },
                    },
                    allocator: buf.allocator,
                }
            }
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
        if let Shared = self.s.tag() {
            unsafe {
                *self.s.shared.counter -= 1;
                if *self.s.shared.counter == 0 {
                    deallocate(&mut self.allocator, self.s.shared.counter);
                }
            }
        }
    }
}

impl<'a> write::String for &'a String {
    fn write<S: Sink>(self, s: &mut S) {
        write::fwrite_str(s, self);
    }
}

#[cfg(test)]
mod tests {
    use super::super::io::read::fread;
    use super::super::io::slice::SliceSource;
    use super::{String, StringBuffer};

    #[test]
    fn test_inline_string_buffer() {
        let source = &mut SliceSource::new(b"abc ");
        let mut buffer: StringBuffer = Default::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abc" as &'static [u8]);
        assert!(s1 == s2);
    }

    #[test]
    fn test_shared_string_buffer() {
        let source = &mut SliceSource::new(b"abcdefghijklmnopqrstuvwxyz");
        let mut buffer: StringBuffer = Default::default();
        fread(source, &mut buffer);
        let s1: String = From::from(buffer);
        let s2: String = From::from(b"abcdefghijklmnopqrstuvwxyz" as &'static [u8]);
        assert!(s1 == s2);
    }
}
