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
        let data = Alloc::alloc_array(&mut allocator, capacity).expect("alloc failed");
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

    pub fn grow(&mut self, n: usize) -> usize {
        let src = usize::checked_sub(self.capacity, n).expect("n greater than capacity");
        let new_capacity = P::grow(self.capacity);
        let grow = usize::checked_sub(new_capacity, self.capacity).expect("grow to a smaller size");
        self.data = unsafe {
            Alloc::realloc_array(&mut self.allocator, self.data, self.capacity, new_capacity)
        }
        .expect("realloc failed");
        let dst = usize::checked_add(src, grow).unwrap_or_else(|| unreachable!());
        self.copy(src, dst, n);
        self.capacity = new_capacity;
        grow
    }

    pub fn shrink(&mut self, size: usize, m: Option<usize>, n: usize) -> usize {
        assert!(n <= size);
        assert!(size <= self.capacity);
        let src = usize::checked_sub(self.capacity, n).unwrap_or_else(|| unreachable!());
        let new_capacity = P::shrink(size, self.capacity);
        let shrink =
            usize::checked_sub(self.capacity, new_capacity).expect("shrink to a bigger size");
        let dst = usize::checked_sub(src, shrink).unwrap_or_else(|| unreachable!());
        if shrink > 0 {
            match m {
                None => self.copy(src, dst, n),
                Some(i) => self.copy(i, 0, n),
            }
            self.data = unsafe {
                Alloc::realloc_array(&mut self.allocator, self.data, self.capacity, new_capacity)
            }
            .expect("realloc failed");
        }
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
        unsafe { Alloc::dealloc_array(&mut self.allocator, self.data, self.capacity) }
            .expect("dealloc failed");
    }
}
