use super::{InlineString, SharedString, String, Union as StringUnion};
use crate::capacity::{DefaultPolicy, Policy};
use crate::io::{PeekableSource, Sink, Source};
use crate::scan::{is_whitespace, Consumer};
use alloc::alloc::{Alloc, Global};
use core::marker::PhantomData;
use core::mem::{forget, size_of, transmute_copy};
use core::ptr::{copy_nonoverlapping, NonNull};
use core::slice::{from_raw_parts, from_raw_parts_mut};

#[cfg(all(target_endian = "little"))]
#[derive(Clone, Copy)]
struct SharedBuffer {
    s: NonNull<u8>,
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
    const fn offset(&self) -> u8 {
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

impl Union {
    fn tag(&self) -> Tag {
        match unsafe { self.inline.offset & 0x3 } {
            0 => Tag::Shared,
            1 => Tag::Inline,
            _ => unreachable!(),
        }
    }

    fn is_inline(&self) -> bool {
        match self.tag() {
            Tag::Shared => false,
            Tag::Inline => true,
        }
    }

    fn len(&self) -> usize {
        unsafe {
            match self.tag() {
                Tag::Shared => self.shared.offset,
                Tag::Inline => (self.inline.offset >> 2) as usize,
            }
        }
    }

    unsafe fn as_ptr(&self) -> *const u8 {
        let counter_size = size_of::<usize>();
        match self.tag() {
            Tag::Shared => self.shared.s.as_ptr().add(counter_size),
            Tag::Inline => self.inline.s.as_ptr(),
        }
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        let counter_size = size_of::<usize>();
        match self.tag() {
            Tag::Shared => self.shared.s.as_ptr().add(counter_size),
            Tag::Inline => self.inline.s.as_mut_ptr(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.as_ptr(), self.len() as usize) }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.as_mut_ptr(), self.len() as usize) }
    }
}

pub struct Buffer<P: Policy = DefaultPolicy, A: Alloc = Global> {
    buffer: Union,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<P: Policy, A: Alloc + Default> Buffer<P, A> {
    pub fn new() -> Self {
        Self {
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

impl<P: Policy, A: Alloc + Default> Default for Buffer<P, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Policy, A: Alloc> AsRef<[u8]> for Buffer<P, A> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_bytes()
    }
}

impl<P: Policy, A: Alloc> AsMut<[u8]> for Buffer<P, A> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buffer.as_bytes_mut()
    }
}

impl<P: Policy, A: Alloc> Sink for Buffer<P, A> {
    fn write(&mut self, c: u8) {
        let counter_size = size_of::<usize>();

        if self.buffer.is_inline() {
            unsafe {
                let offset = self.buffer.inline.offset() as usize;
                let size = self.buffer.inline.s.len();
                if offset < size {
                    self.buffer.inline.s[offset] = c;
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        self.buffer.inline.offset = ((offset as u8 + 1) << 2) | 1;
                    }
                    return;
                } else {
                    let capacity = P::initial(size);
                    let s: NonNull<u8> =
                        Alloc::alloc_array::<u8>(&mut self.allocator, counter_size + capacity)
                            .expect("alloc failed");
                    copy_nonoverlapping(
                        self.buffer.inline.s.as_ptr(),
                        s.as_ptr().add(counter_size),
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
                self.buffer.shared.s = Alloc::realloc_array(
                    &mut self.allocator,
                    self.buffer.shared.s,
                    counter_size + self.buffer.shared.capacity,
                    counter_size + capacity,
                )
                .expect("realloc failed");
            }
            *self.buffer.shared.s.as_ptr().add(counter_size + offset) = c;
            self.buffer.shared.offset += 1;
        }
    }
}

impl<'a, P: Policy, A: Alloc> Consumer for &'a mut Buffer<P, A> {
    fn consume<I: Source>(self, s: &mut PeekableSource<I>) -> bool {
        while let Some(&c) = s.peek() {
            if is_whitespace(c) {
                break;
            }

            Sink::write(self, c);
            s.consume();
        }

        true
    }
}

impl<P: Policy, A: Alloc> Drop for Buffer<P, A> {
    fn drop(&mut self) {
        if !self.buffer.is_inline() {
            let counter_size = size_of::<usize>();
            unsafe {
                Alloc::dealloc_array(
                    &mut self.allocator,
                    self.buffer.shared.s,
                    counter_size + self.buffer.shared.capacity,
                )
                .expect("dealloc failed");
            }
        }
    }
}

struct BufferTransmute<P: Policy, A: Alloc> {
    buffer: Union,
    allocator: A,
    _policy: PhantomData<P>,
}

#[allow(clippy::fallible_impl_from)]
impl<P: Policy, A: Alloc> From<Buffer<P, A>> for String<A> {
    fn from(x: Buffer<P, A>) -> Self {
        let mut buf: BufferTransmute<P, A> = unsafe { transmute_copy(&x) };
        forget(x);

        let counter_size = size_of::<usize>();

        if buf.buffer.is_inline() {
            unsafe {
                Self {
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
                let s = Alloc::realloc_array(
                    &mut buf.allocator,
                    buf.buffer.shared.s,
                    counter_size + buf.buffer.shared.capacity,
                    counter_size + length,
                )
                .expect("realloc failed");
                // #[allow(clippy::cast_ptr_alignment)]
                let mut counter = NonNull::cast::<usize>(s);
                *counter.as_mut() = 1;

                Self {
                    s: StringUnion {
                        shared: SharedString {
                            counter,
                            length,
                            s: NonNull::new(s.as_ptr().add(counter_size)).unwrap(),
                        },
                    },
                    allocator: buf.allocator,
                }
            }
        }
    }
}
