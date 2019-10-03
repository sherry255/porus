use super::Tag::{Inline, Shared, Static};
use super::{InlineString, SharedString, String, Union};
use crate::capacity::{DefaultPolicy, Policy};
use crate::io::{PeekableSource, Sink, Source};
use crate::scan::{is_whitespace, Consumer};
use crate::utils::unwrap;
use alloc::alloc::{Alloc, Global};
use core::marker::PhantomData;
use core::mem::{forget, size_of, transmute_copy};
use core::ptr::{copy_nonoverlapping, NonNull};
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub struct Buffer<P: Policy = DefaultPolicy, A: Alloc = Global> {
    buffer: Union,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<P: Policy, A: Alloc + Default> Buffer<P, A> {
    pub fn new() -> Self {
        Self {
            buffer: Union {
                inline: InlineString {
                    length: 1,
                    s: [0; size_of::<Union>() - 1],
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

unsafe fn as_ptr(u: &Union) -> *const u8 {
    match u.tag() {
        Inline => u.inline.s.as_ptr(),
        Shared => u.shared.counter.as_ptr().add(1).cast(),
        Static => unreachable!(),
    }
}

unsafe fn as_mut_ptr(u: &mut Union) -> *mut u8 {
    match u.tag() {
        Inline => u.inline.s.as_mut_ptr(),
        Shared => u.shared.counter.as_ptr().add(1).cast(),
        Static => unreachable!(),
    }
}

unsafe fn capacity(u: &Union) -> usize {
    match u.tag() {
        Inline => u.inline.s.len(),
        Shared => u.shared.length,
        Static => unreachable!(),
    }
}

unsafe fn resize<A: Alloc>(allocator: &mut A, s: &mut SharedString, new_size: usize) {
    let counter_size = size_of::<usize>();
    s.counter = Alloc::realloc_array::<u8>(
        allocator,
        s.counter.cast(),
        usize::wrapping_add(counter_size, s.length),
        usize::wrapping_add(counter_size, new_size),
    )
    .expect("realloc failed")
    .cast();
    s.length = new_size;
}

#[allow(clippy::cast_sign_loss)]
fn len(u: &Union) -> usize {
    match u.tag() {
        Inline => u.len(),
        Shared => unsafe { u.shared.s.as_ptr().offset_from(as_ptr(u)) as usize },
        Static => unreachable!(),
    }
}

impl<P: Policy, A: Alloc> AsRef<[u8]> for Buffer<P, A> {
    fn as_ref(&self) -> &[u8] {
        let buf = &self.buffer;
        unsafe { from_raw_parts(as_ptr(buf), len(buf)) }
    }
}

impl<P: Policy, A: Alloc> AsMut<[u8]> for Buffer<P, A> {
    fn as_mut(&mut self) -> &mut [u8] {
        let buf = &mut self.buffer;
        unsafe { from_raw_parts_mut(as_mut_ptr(buf), len(buf)) }
    }
}

impl<P: Policy, A: Alloc> Sink for Buffer<P, A> {
    fn write(&mut self, c: u8) {
        let offset = len(&self.buffer);
        let capacity = unsafe { capacity(&self.buffer) };

        match self.buffer.tag() {
            Inline => unsafe {
                if offset < capacity {
                    *self.buffer.inline.s.get_unchecked_mut(offset) = c;

                    #[allow(clippy::integer_arithmetic)]
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        self.buffer.inline.length = ((offset as u8 + 1) << 2) | 1;
                    }
                } else {
                    let counter_size = size_of::<usize>();
                    let new_capacity = P::grow(P::initial(capacity));
                    let s = Alloc::alloc_array::<u8>(
                        &mut self.allocator,
                        usize::wrapping_add(counter_size, new_capacity),
                    )
                    .expect("alloc failed");

                    copy_nonoverlapping(
                        self.buffer.as_ptr(),
                        s.as_ptr().add(counter_size),
                        capacity,
                    );

                    self.buffer.shared.counter = s.cast();
                    self.buffer.shared.length = new_capacity;
                    self.buffer.shared.s =
                        unwrap(NonNull::new(as_mut_ptr(&mut self.buffer).add(capacity)));
                    Sink::write(self, c)
                }
            },
            Shared => unsafe {
                if self.buffer.as_ptr() == as_ptr(&self.buffer).add(capacity) {
                    resize(
                        &mut self.allocator,
                        &mut self.buffer.shared,
                        P::grow(capacity),
                    );
                    self.buffer.shared.s =
                        unwrap(NonNull::new(as_mut_ptr(&mut self.buffer).add(offset)));
                }

                *self.buffer.shared.s.as_mut() = c;
                self.buffer.shared.s = unwrap(NonNull::new(self.buffer.shared.s.as_ptr().add(1)));
            },
            Static => unreachable!(),
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
        if let Shared = self.buffer.tag() {
            unsafe {
                Alloc::dealloc_array::<u8>(
                    &mut self.allocator,
                    self.buffer.shared.counter.cast(),
                    usize::wrapping_add(size_of::<usize>(), capacity(&self.buffer)),
                )
                .expect("dealloc failed");
            }
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl<P: Policy, A: Alloc> From<Buffer<P, A>> for String<A> {
    fn from(mut x: Buffer<P, A>) -> Self {
        unsafe {
            if let Shared = x.buffer.tag() {
                let length = len(&x.buffer);
                resize(&mut x.allocator, &mut x.buffer.shared, length);
                x.buffer.shared.s = unwrap(NonNull::new(as_mut_ptr(&mut x.buffer)));
                *x.buffer.shared.counter.as_mut() = 1;
            }

            let s = transmute_copy(&x);

            #[allow(clippy::mem_forget)]
            {
                forget(x);
            }
            s
        }
    }
}
