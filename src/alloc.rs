use core::fmt::Debug;
use core::mem::size_of;
use core::ptr::{null_mut, read, write, NonNull};
use porus::pool::{self, Pool};

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

impl<T, A: Allocator> Pool<T> for A {
    type Handle = Handle;

    fn get(&self, handle: Self::Handle) -> &T {
        unsafe { &*handle.0.cast().as_ptr() }
    }

    fn get_mut(&mut self, handle: Self::Handle) -> &mut T {
        unsafe { &mut *handle.0.cast().as_ptr() }
    }

    fn add(&mut self, item: T) -> Self::Handle {
        unsafe {
            let ptr = allocate(self, 1);
            write(ptr, item);
            Handle(NonNull::new(ptr as *mut _).unwrap())
        }
    }

    fn remove(&mut self, handle: Self::Handle) -> T {
        unsafe {
            let item = read(handle.0.cast().as_ptr());
            deallocate(self, handle.0.as_ptr());
            item
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Handle;
    use core::mem::size_of;

    #[test]
    fn test_handle_size() {
        assert!(size_of::<Handle>() == size_of::<Option<Handle>>());
    }
}
