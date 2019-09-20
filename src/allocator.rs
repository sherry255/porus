use crate::libc;
use crate::pool;
use alloc::alloc::{Alloc, Global, GlobalAlloc, Layout};
use core::cmp::min;
use core::marker::PhantomData;
use core::ptr::{copy_nonoverlapping, null_mut, read, write, NonNull};

#[derive(Copy, Clone)]
pub struct System;

// libstd/sys_common/alloc.rs
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
pub const MIN_ALIGN: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
pub const MIN_ALIGN: usize = 16;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub unsafe fn realloc_fallback(
    alloc: &System,
    ptr: *mut u8,
    old_layout: Layout,
    new_size: usize,
) -> *mut u8 {
    // Docs for GlobalAlloc::realloc require this to be valid:
    let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());

    let new_ptr = GlobalAlloc::alloc(alloc, new_layout);
    if !new_ptr.is_null() {
        let size = min(old_layout.size(), new_size);
        copy_nonoverlapping(ptr, new_ptr, size);
        GlobalAlloc::dealloc(alloc, ptr, old_layout);
    }
    new_ptr
}

// libstd/sys/unix/alloc.rs
unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
    let mut out = null_mut();
    let ret = libc::posix_memalign(&mut out, layout.align(), layout.size());
    if ret == 0 {
        out
    } else {
        null_mut()
    }
}

unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            libc::malloc(layout.size())
        } else {
            aligned_malloc(&layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
            libc::realloc(ptr, new_size)
        } else {
            realloc_fallback(self, ptr, layout, new_size)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Handle(NonNull<u8>);

impl pool::Handle for Handle {}

pub struct Pool<T, A: Alloc = Global> {
    allocator: A,
    _type: PhantomData<T>,
}

impl<T, A: Alloc> Pool<T, A> {
    pub fn new_with_allocator(allocator: A) -> Self {
        Self {
            allocator,
            _type: PhantomData,
        }
    }
}

impl<T, A: Alloc + Default> Pool<T, A> {
    pub fn new() -> Self {
        Self::new_with_allocator(Default::default())
    }
}

impl<T, A: Alloc + Default> Default for Pool<T, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Alloc> pool::Pool for Pool<T, A> {
    type Handle = Handle;
    type Elem = T;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &*handle.0.cast().as_ptr() }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut *handle.0.cast().as_ptr() }
    }

    fn add(&mut self, item: T) -> Handle {
        unsafe {
            let ptr = Alloc::alloc_one::<T>(&mut self.allocator).expect("alloc faield");
            write(ptr.as_ptr(), item);
            Handle(NonNull::cast(ptr))
        }
    }

    fn remove(&mut self, handle: Handle) -> T {
        unsafe {
            let item = read(handle.0.cast().as_ptr());
            Alloc::dealloc_one(&mut self.allocator, handle.0);
            item
        }
    }
}
