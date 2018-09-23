use core::iter::{ExactSizeIterator, Iterator};
use crate::allocator::Allocator;
use crate::block::Block;
use crate::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use crate::collection::Collection;
use crate::list::{List, ListMut};
use crate::os;
use crate::stack::Stack;

pub struct Array<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = os::Allocator> {
    size: usize,
    data: Block<T, P, A>,
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Array<T, P, A> {
    pub fn new_from_iter<I: ExactSizeIterator<Item = T>>(mut it: I) -> Self {
        let size = ExactSizeIterator::len(&it);
        let mut data = Block::new(Default::default(), size);
        assert!(data.capacity() >= size);

        for i in 0..size {
            data.write(i, Iterator::next(&mut it).unwrap());
        }

        Array { size, data }
    }
}

impl<T: Clone, P: CapacityPolicy, A: Allocator + Default> Array<T, P, A> {
    pub fn new_from_elem(x: T, size: isize) -> Self {
        Array::new_from_iter((0..size).map(|_| Clone::clone(&x)))
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Collection for Array<T, P, A> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<T, P: CapacityPolicy, A: Allocator> List for Array<T, P, A> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            Some(self.data.get(index))
        } else {
            None
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> ListMut for Array<T, P, A> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.size {
            Some(self.data.get_mut(index))
        } else {
            None
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Stack for Array<T, P, A> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, elem: T) {
        if self.size == self.data.capacity() {
            assert!(self.data.grow(0) > 0);
        }

        self.data.write(self.size, elem);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            let item = self.data.read(self.size);
            self.data.shrink(self.size, None, 0);
            Some(item)
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Array<T, P, A> {
    fn drop(&mut self) {
        for i in 0..self.size {
            self.data.read(i);
        }
    }
}

pub macro array($elem:expr; $n:expr) {
    &mut Array::<_>::new_from_elem($elem, $n)
}

#[cfg(test)]
mod tests {
    use super::Array;
    use core::cell::Cell;
    use crate::tests::Item;

    #[test]
    fn test_drop() {
        let counter = Cell::new(0);
        {
            Array::<_>::new_from_iter((0..10).map(|_| Item::new(&counter)));
        }
        assert!(counter.into_inner() == 10);
    }
}
