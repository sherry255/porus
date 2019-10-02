extern crate porus;

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
