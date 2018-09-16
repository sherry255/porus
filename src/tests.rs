use core::cell::Cell;

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
