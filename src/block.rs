use core::marker::PhantomData;
use core::ptr::{copy, read, write};
use crate::allocator::{allocate, deallocate, reallocate, Allocator};
use crate::capacity::Policy;

pub struct Block<T, P: Policy, A: Allocator> {
    capacity: usize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: Policy, A: Allocator> Block<T, P, A> {
    pub fn new(mut allocator: A, size: usize) -> Self {
        let capacity = P::initial(size);
        let data = unsafe { allocate(&mut allocator, capacity) };
        Self {
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
        unsafe { read(self.data.add(index)) }
    }

    pub fn write(&mut self, index: usize, item: T) {
        assert!(index < self.capacity);
        unsafe { write(self.data.add(index), item) }
    }

    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.capacity);
        unsafe { &*self.data.add(index) }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.capacity);
        unsafe { &mut *self.data.add(index) }
    }

    pub fn copy(&mut self, src: usize, dst: usize, count: usize) {
        unsafe { copy(self.data.add(src), self.data.add(dst), count) }
    }

    fn move_tail(&mut self, new_capacity: usize, n: usize) {
        let src = self.capacity - n;
        self.copy(src, new_capacity - n, n);
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

impl<T, P: Policy, A: Allocator + Default> Block<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self::new(Default::default(), capacity)
    }
}

impl<T, P: Policy, A: Allocator> Drop for Block<T, P, A> {
    fn drop(&mut self) {
        unsafe { deallocate(&mut self.allocator, self.data) };
    }
}
