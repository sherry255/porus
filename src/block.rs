use crate::capacity::Policy;
use core::alloc::Alloc;
use core::marker::PhantomData;
use core::ptr::{copy, read, write, NonNull};

pub struct Block<T, P: Policy, A: Alloc> {
    capacity: usize,
    data: NonNull<T>,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: Policy, A: Alloc> Block<T, P, A> {
    pub fn new(mut allocator: A, size: usize) -> Self {
        let capacity = P::initial(size);
        let data = Alloc::alloc_array(&mut allocator, capacity).unwrap();
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
        unsafe { read(self.data.as_ptr().add(index)) }
    }

    pub fn write(&mut self, index: usize, item: T) {
        assert!(index < self.capacity);
        unsafe { write(self.data.as_ptr().add(index), item) }
    }

    pub fn get(&self, index: usize) -> &T {
        assert!(index < self.capacity);
        unsafe { &*self.data.as_ptr().add(index) }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.capacity);
        unsafe { &mut *self.data.as_ptr().add(index) }
    }

    pub fn copy(&mut self, src: usize, dst: usize, count: usize) {
        unsafe {
            copy(
                self.data.as_ptr().add(src),
                self.data.as_ptr().add(dst),
                count,
            )
        }
    }

    fn move_tail(&mut self, new_capacity: usize, n: usize) {
        let src = self.capacity - n;
        self.copy(src, new_capacity - n, n);
    }

    pub fn grow(&mut self, n: usize) -> usize {
        assert!(n <= self.capacity);
        let new_capacity = P::grow(self.capacity);
        assert!(self.capacity <= new_capacity);
        self.data = unsafe {
            Alloc::realloc_array(&mut self.allocator, self.data, self.capacity, new_capacity)
        }
        .unwrap();
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
            self.data = unsafe {
                Alloc::realloc_array(&mut self.allocator, self.data, self.capacity, new_capacity)
            }
            .unwrap();
        }
        let shrink = self.capacity - new_capacity;
        self.capacity = new_capacity;
        shrink
    }
}

impl<T, P: Policy, A: Alloc + Default> Block<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self::new(Default::default(), capacity)
    }
}

impl<T, P: Policy, A: Alloc> Drop for Block<T, P, A> {
    fn drop(&mut self) {
        unsafe { Alloc::dealloc_array(&mut self.allocator, self.data, self.capacity) }.unwrap();
    }
}
