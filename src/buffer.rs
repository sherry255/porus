use crate::block::Block;
use crate::capacity::{DefaultPolicy, Policy};
use crate::collection::Collection;
use crate::deque::Deque;
use crate::list::{List, ListMut};
use alloc::alloc::{Alloc, Global};

pub struct Buffer<T, P: Policy = DefaultPolicy, A: Alloc = Global> {
    front: usize,
    back: usize,
    data: Block<T, P, A>,
}

impl<T, P: Policy, A: Alloc + Default> Buffer<T, P, A> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        let data = Block::new(Default::default(), capacity + 1);

        Self {
            front: 0,
            back: 0,
            data,
        }
    }
}

impl<T, P: Policy, A: Alloc + Default> Default for Buffer<T, P, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, P: Policy, A: Alloc> Buffer<T, P, A> {
    fn increase_index(&self, index: usize) -> usize {
        assert!(index < self.data.capacity());
        if index + 1 == self.data.capacity() {
            0
        } else {
            index + 1
        }
    }

    fn decrease_index(&self, index: usize) -> usize {
        assert!(index < self.data.capacity());
        if index == 0 {
            self.data.capacity() - 1
        } else {
            index - 1
        }
    }

    fn is_full(&self) -> bool {
        self.increase_index(self.back) == self.front
    }

    fn grow(&mut self) {
        if self.front <= self.back {
            assert!(self.data.grow(0) > 0);
        } else {
            let capacity = self.data.capacity();
            let grow = self.data.grow(capacity - self.front);
            self.front += grow;
            assert!(grow > 0);
        }
    }

    fn shrink(&mut self) {
        let size = Collection::size(self);
        if self.front <= self.back {
            let shrink = self.data.shrink(size, Some(self.front), size);
            if shrink > 0 {
                self.front = 0;
                self.back = size;
            }
        } else {
            let capacity = self.data.capacity();
            let shrink = self.data.shrink(size, None, capacity - self.front);
            self.front -= shrink;
        }
    }

    fn index(&self, index: usize) -> Option<usize> {
        if index < Collection::size(self) {
            if self.front + index < self.data.capacity() {
                Some(self.front + index)
            } else {
                Some(self.front + index - self.data.capacity())
            }
        } else {
            None
        }
    }
}

impl<T, P: Policy, A: Alloc> Collection for Buffer<T, P, A> {
    fn size(&self) -> usize {
        if self.front <= self.back {
            self.back - self.front
        } else {
            self.back + self.data.capacity() - self.front
        }
    }
}

impl<T, P: Policy, A: Alloc> List for Buffer<T, P, A> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        match self.index(index) {
            None => None,
            Some(i) => Some(self.data.get(i)),
        }
    }
}

impl<T, P: Policy, A: Alloc> ListMut for Buffer<T, P, A> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.index(index) {
            None => None,
            Some(i) => Some(self.data.get_mut(i)),
        }
    }
}

impl<T, P: Policy, A: Alloc> Deque for Buffer<T, P, A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.front == self.back
    }

    fn push_front(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.front = self.decrease_index(self.front);
        self.data.write(self.front, elem);
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let elem = self.data.read(self.front);
            self.front = self.increase_index(self.front);
            self.shrink();
            Some(elem)
        }
    }

    fn push_back(&mut self, elem: T) {
        if self.is_full() {
            self.grow();
        }

        self.data.write(self.back, elem);
        self.back = self.increase_index(self.back);
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.back = self.decrease_index(self.back);
            let elem = self.data.read(self.back);
            self.shrink();
            Some(elem)
        }
    }

    fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.get(self.front))
        }
    }

    fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.get(self.decrease_index(self.back)))
        }
    }
}

impl<T, P: Policy, A: Alloc> Drop for Buffer<T, P, A> {
    fn drop(&mut self) {
        if self.back < self.front {
            for i in 0..self.back {
                self.data.read(i);
            }

            for i in self.front..self.data.capacity() {
                self.data.read(i);
            }
        } else {
            for i in self.front..self.back {
                self.data.read(i);
            }
        }
    }
}

pub macro buffer() {
    &mut Buffer::<_>::new()
}
