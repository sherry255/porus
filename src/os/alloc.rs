use super::libc::{free, get_error, malloc, realloc};
use super::OSError;
use core::ptr::null_mut;
use porus::allocator;

pub struct Allocator {}

impl allocator::Allocator for Allocator {
    type Error = OSError;

    unsafe fn reallocate(&mut self, ptr: *mut u8, size: usize) -> Result<*mut u8, OSError> {
        if size == 0 {
            if !ptr.is_null() {
                free(ptr);
            }
            Ok(null_mut())
        } else {
            let p = if ptr.is_null() {
                malloc(size)
            } else {
                realloc(ptr, size)
            };

            if p.is_null() {
                get_error()
            } else {
                Ok(p)
            }
        }
    }
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {}
    }
}

impl Default for Allocator {
    fn default() -> Self {
        Allocator::new()
    }
}
