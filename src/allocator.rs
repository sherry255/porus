use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::{null_mut, read, write, NonNull};
use crate::os;
use crate::pool;

pub trait Allocator {
    type Error: Debug;

    unsafe fn reallocate(&mut self, ptr: *mut u8, capacity: usize) -> Result<*mut u8, Self::Error>;
}

pub unsafe fn reallocate<T, A: Allocator>(
    allocator: &mut A,
    ptr: *mut T,
    capacity: usize,
) -> *mut T {
    let size = size_of::<T>();
    Allocator::reallocate(allocator, ptr as *mut _, size * capacity).unwrap() as *mut _
}

pub unsafe fn allocate<T, A: Allocator>(allocator: &mut A, capacity: usize) -> *mut T {
    reallocate(allocator, null_mut(), capacity)
}

pub unsafe fn deallocate<T, A: Allocator>(allocator: &mut A, ptr: *mut T) {
    reallocate(allocator, ptr, 0);
}

#[derive(Clone, Copy)]
pub struct Handle(NonNull<u8>);

impl pool::Handle for Handle {}

pub struct Pool<T, A: Allocator = os::Allocator> {
    allocator: A,
    _type: PhantomData<T>,
}

impl<T, A: Allocator> Pool<T, A> {
    pub fn new_with_allocator(allocator: A) -> Self {
        Self {
            allocator,
            _type: PhantomData,
        }
    }
}

impl<T, A: Allocator + Default> Pool<T, A> {
    pub fn new() -> Self {
        Self::new_with_allocator(Default::default())
    }
}

impl<T, A: Allocator + Default> Default for Pool<T, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Allocator> pool::Pool for Pool<T, A> {
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
            let ptr = allocate(&mut self.allocator, 1);
            write(ptr, item);
            Handle(NonNull::new(ptr as *mut _).unwrap())
        }
    }

    fn remove(&mut self, handle: Handle) -> T {
        unsafe {
            let item = read(handle.0.cast().as_ptr());
            deallocate(&mut self.allocator, handle.0.as_ptr());
            item
        }
    }
}
