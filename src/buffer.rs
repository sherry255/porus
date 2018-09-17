use core::marker::PhantomData;
use porus::alloc::{allocate, deallocate, reallocate, Allocator};
use porus::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use porus::collection::Collection;
use porus::deque::Deque;
use porus::list::{List, ListMut};
use porus::os;
use porus::ptr::{copy, get, get_mut, read, write};

pub struct Buffer<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = os::Allocator> {
    front: usize,
    back: usize,
    capacity: usize,
    data: *mut T,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Buffer<T, P, A> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        let capacity = P::initial(capacity) + 1;
        let mut allocator = Default::default();
        let data = unsafe { allocate(&mut allocator, capacity) };
        Buffer {
            front: 0,
            back: 0,
            capacity,
            data,
            allocator,
            _policy: PhantomData,
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Default for Buffer<T, P, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Buffer<T, P, A> {
    fn increase_index(&self, index: usize) -> usize {
        if index + 1 == self.capacity {
            0
        } else {
            index + 1
        }
    }

    fn decrease_index(&self, index: usize) -> usize {
        if index == 0 {
            self.capacity - 1
        } else {
            index - 1
        }
    }

    fn grow_to(&mut self, new_capacity: usize) {
        self.data = unsafe { reallocate(&mut self.allocator, self.data, new_capacity) };
        if self.back < self.front {
            let grow = new_capacity - self.capacity;
            unsafe {
                copy(
                    self.data,
                    self.front,
                    self.front + grow,
                    self.capacity - self.front,
                )
            };
            self.front += grow;
        }
        self.capacity = new_capacity;
    }

    fn shrink_to(&mut self, new_capacity: usize) {
        if self.back < self.front {
            let shrink = self.capacity - new_capacity;
            unsafe {
                copy(
                    self.data,
                    self.front,
                    self.front - shrink,
                    self.capacity - self.front,
                )
            };
            self.front -= shrink;
        } else if self.back > new_capacity {
            let size = self.back - self.front;
            unsafe { copy(self.data, self.front, 0, size) };
            self.front = 0;
            self.back = size;
        }

        self.data = unsafe { reallocate(&mut self.allocator, self.data, new_capacity) };
        self.capacity = new_capacity;
    }

    fn is_full(&self) -> bool {
        self.increase_index(self.back) == self.front
    }

    fn grow(&mut self) {
        let new_capacity = P::grow(self.capacity - 1) + 1;
        self.grow_to(new_capacity);
    }

    fn shrink(&mut self) {
        let new_capacity = P::shrink(Collection::size(self), self.capacity - 1) + 1;
        if new_capacity < self.capacity {
            self.shrink_to(new_capacity);
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Collection for Buffer<T, P, A> {
    fn size(&self) -> usize {
        if self.front <= self.back {
            self.back - self.front
        } else {
            self.back + self.capacity - self.front
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> List for Buffer<T, P, A> {
    type Elem = T;

    #[cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
    fn get(&self, index: usize) -> Option<&T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(unsafe { get(self.data, self.front + index) })
            }
        } else {
            if self.front + index >= self.back + self.capacity {
                None
            } else if self.front + index >= self.capacity {
                Some(unsafe { get(self.data, self.front + index - self.capacity) })
            } else {
                Some(unsafe { get(self.data, self.front + index) })
            }
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListMut for Buffer<T, P, A> {
    #[cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if self.front <= self.back {
            if self.front + index >= self.back {
                None
            } else {
                Some(unsafe { get_mut(self.data, self.front + index) })
            }
        } else {
            if self.front + index >= self.back + self.capacity {
                None
            } else if self.front + index >= self.capacity {
                Some(unsafe { get_mut(self.data, self.front + index - self.capacity) })
            } else {
                Some(unsafe { get_mut(self.data, self.front + index) })
            }
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Deque for Buffer<T, P, A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front == self.back
    }

    fn push_front(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.front = self.decrease_index(self.front);
        unsafe { write(self.data, self.front, elem) };
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let elem = unsafe { read(self.data, self.front) };
            self.front = self.increase_index(self.front);
            self.shrink();
            Some(elem)
        }
    }

    fn push_back(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        unsafe { write(self.data, self.back, elem) };
        self.back = self.increase_index(self.back);
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.back = self.decrease_index(self.back);
            let elem = unsafe { read(self.data, self.back) };
            self.shrink();
            Some(elem)
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Buffer<T, P, A> {
    fn drop(&mut self) {
        if self.back < self.front {
            for i in 0..self.back {
                unsafe { read(self.data, i) };
            }

            for i in self.front..self.capacity {
                unsafe { read(self.data, i) };
            }
        } else {
            for i in self.front..self.back {
                unsafe { read(self.data, i) };
            }
        }
        unsafe { deallocate(&mut self.allocator, self.data) };
    }
}

#[macro_export]
macro_rules! buffer {
    () => {
        &mut $crate::buffer::Buffer::<_, $crate::capacity::DefaultCapacityPolicy>::new()
    };
}
