use core::marker::PhantomData;
use core::ptr::{copy, read, write};
use crate::allocator::{allocate, deallocate, reallocate, Allocator};
use crate::capacity::CapacityPolicy;

pub struct Block<T, P: CapacityPolicy, A: Allocator> {
    capacity: usize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: CapacityPolicy, A: Allocator> Block<T, P, A> {
    pub fn new(mut allocator: A, capacity: usize) -> Self {
        let capacity = P::initial(capacity);
        let data = unsafe { allocate(&mut allocator, capacity) };
        Block {
            capacity,
            data,
            allocator,
            _policy: PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn read(&mut self, index: usize) -> T {
        assert!(index < self.capacity);
        unsafe { read(self.data.offset(index as isize)) }
    }

    pub fn write(&mut self, index: usize, item: T) {
        assert!(index < self.capacity);
        unsafe { write(self.data.offset(index as isize), item) }
    }

    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.capacity);
        unsafe { &*self.data.offset(index as isize) }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.capacity);
        unsafe { &mut *self.data.offset(index as isize) }
    }

    pub fn copy(&mut self, src: usize, dst: usize, count: usize) {
        unsafe {
            copy(
                self.data.offset(src as isize),
                self.data.offset(dst as isize),
                count,
            )
        }
    }

    fn move_tail(&mut self, new_capacity: usize, n: usize) {
        self.copy(self.capacity - n, new_capacity - n, n);
    }

    pub fn grow(&mut self, n: usize) -> usize {
        assert!(n <= self.capacity);
        let new_capacity = P::grow(self.capacity);
        assert!(self.capacity <= new_capacity);
        self.data = unsafe { reallocate(&mut self.allocator, self.data, new_capacity) };
        self.move_tail(new_capacity, n);
        let grow = new_capacity - self.capacity;
        self.capacity = new_capacity;
        grow
    }

    pub fn shrink(&mut self, size: usize, m: Option<usize>, n: usize) -> usize {
        assert!(n <= size);
        let new_capacity = P::shrink(size, self.capacity);
        assert!(size <= new_capacity);
        assert!(new_capacity <= self.capacity);
        if new_capacity < self.capacity {
            match m {
                None => self.move_tail(new_capacity, n),
                Some(i) => self.copy(i, 0, n),
            }
            self.data = unsafe { reallocate(&mut self.allocator, self.data, new_capacity) };
        }
        let shrink = self.capacity - new_capacity;
        self.capacity = new_capacity;
        shrink
    }
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Block<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Block::new(Default::default(), capacity)
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Block<T, P, A> {
    fn drop(&mut self) {
        unsafe { deallocate(&mut self.allocator, self.data) };
    }
}
