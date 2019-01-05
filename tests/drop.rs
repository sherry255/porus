extern crate porus;
use porus::prelude::*;

use std::cell::Cell;

pub struct Item<'a> {
    counter: &'a Cell<usize>,
}

impl<'a> Item<'a> {
    pub fn new(counter: &'a Cell<usize>) -> Self {
        Item { counter }
    }
}

impl<'a> Drop for Item<'a> {
    fn drop(&mut self) {
        self.counter.replace(self.counter.get() + 1);
    }
}

trait CollectionDropTest {
    fn test_drop_impl<'a, It: ExactSizeIterator<Item = Item<'a>>>(it: It);

    fn test_drop() {
        let counter = Cell::new(0);
        Self::test_drop_impl((0..10).map(|_| Item::new(&counter)));
        assert!(counter.into_inner() == 10);
    }
}

struct TestAllocPool {}

impl CollectionDropTest for TestAllocPool {
    fn test_drop_impl<'a, It: ExactSizeIterator<Item = Item<'a>>>(it: It) {
        let p = &mut allocator::Pool::<_>::new();
        let handles = &mut Array::<_>::new_from_iter(it.map(|item| pool::add(p, item)));
        list::iter(handles).for_each(|h| {
            pool::remove(p, h);
        });
    }
}

struct TestChunk {}

impl CollectionDropTest for TestChunk {
    fn test_drop_impl<'a, It: ExactSizeIterator<Item = Item<'a>>>(it: It) {
        let p = &mut Chunk::<_>::new_with_capacity(0);
        let handles = &mut Array::<_>::new_from_iter(it.map(|item| pool::add(p, item)));
        list::iter(handles).for_each(|h| {
            pool::remove(p, h);
        });
    }
}

struct TestArray {}

impl CollectionDropTest for TestArray {
    fn test_drop_impl<'a, It: ExactSizeIterator<Item = Item<'a>>>(it: It) {
        Array::<_>::new_from_iter(it);
    }
}

#[test]
fn test_alloc_pool_drop() {
    TestAllocPool::test_drop();
}

#[test]
fn test_chunk_drop() {
    TestChunk::test_drop();
}

#[test]
fn test_array_drop() {
    TestArray::test_drop();
}
