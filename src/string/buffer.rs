use super::{InlineString, SharedString, String, Union as StringUnion};
use core::marker::PhantomData;
use core::mem::{forget, size_of, transmute_copy};
use core::ptr::copy_nonoverlapping;
use core::slice::from_raw_parts;
use crate::allocator::{allocate, deallocate, reallocate, Allocator};
use crate::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use crate::io::{PeekableSource, Sink, Source};
use crate::os;
use crate::scan::{is_whitespace, Consumer};

#[cfg(all(target_endian = "little"))]
#[derive(Clone, Copy)]
struct SharedBuffer {
    s: *mut u8,
    capacity: usize,
    offset: usize,
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[derive(Clone, Copy)]
struct InlineBuffer {
    offset: u8,
    s: [u8; 23],
}

impl InlineBuffer {
    fn offset(&self) -> u8 {
        self.offset >> 2
    }
}

union Union {
    shared: SharedBuffer,
    inline: InlineBuffer,
}

enum Tag {
    Shared,
    Inline,
}

use self::Tag::*;

impl Union {
    fn tag(&self) -> Tag {
        match unsafe { self.inline.offset & 0x3 } {
            0 => Shared,
            1 => Inline,
            _ => unreachable!(),
        }
    }

    fn is_inline(&self) -> bool {
        match self.tag() {
            Shared => false,
            Inline => true,
        }
    }

    fn len(&self) -> usize {
        unsafe {
            match self.tag() {
                Shared => self.shared.offset,
                Inline => (self.inline.offset >> 2) as usize,
            }
        }
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        match self.tag() {
            Shared => self.shared.s,
            Inline => self.inline.s.as_ptr(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len() as usize) }
    }
}

pub struct Buffer<P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = os::Allocator> {
    buffer: Union,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<P: CapacityPolicy, A: Allocator + Default> Buffer<P, A> {
    pub fn new() -> Self {
        Buffer {
            buffer: Union {
                inline: InlineBuffer {
                    offset: 1,
                    s: [0; 23],
                },
            },
            allocator: Default::default(),
            _policy: PhantomData,
        }
    }
}

impl<P: CapacityPolicy, A: Allocator + Default> Default for Buffer<P, A> {
    fn default() -> Self {
        Buffer::new()
    }
}

impl<P: CapacityPolicy, A: Allocator> AsRef<[u8]> for Buffer<P, A> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_bytes()
    }
}

impl<P: CapacityPolicy, A: Allocator> Sink for Buffer<P, A> {
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
                    self.buffer.shared = SharedBuffer {
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

impl<'a, P: CapacityPolicy, A: Allocator> Consumer for &'a mut Buffer<P, A> {
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

impl<P: CapacityPolicy, A: Allocator> Drop for Buffer<P, A> {
    fn drop(&mut self) {
        if !self.buffer.is_inline() {
            unsafe {
                deallocate(&mut self.allocator, self.buffer.shared.s);
            }
        }
    }
}

struct BufferTransmute<P: CapacityPolicy, A: Allocator> {
    buffer: Union,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<P: CapacityPolicy, A: Allocator> From<Buffer<P, A>> for String<A> {
    fn from(x: Buffer<P, A>) -> Self {
        let mut buf: BufferTransmute<P, A> = unsafe { transmute_copy(&x) };
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
