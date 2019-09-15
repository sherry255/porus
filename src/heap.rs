pub trait Heap {
    type Elem;

    fn push(&mut self, item: Self::Elem);
    fn pop(&mut self) -> Option<Self::Elem>;
    fn peek(&self) -> Option<&Self::Elem>;
}

pub fn push<H: Heap>(heap: &mut H, item: H::Elem) {
    Heap::push(heap, item);
}

pub fn pop<H: Heap>(heap: &mut H) -> H::Elem {
    Heap::pop(heap).unwrap()
}

pub fn peek<H: Heap>(heap: &mut H) -> &H::Elem {
    Heap::peek(heap).unwrap()
}

use alloc::collections::BinaryHeap;

impl<T: Ord> Heap for BinaryHeap<T> {
    type Elem = T;

    fn push(&mut self, item: Self::Elem) {
        self.push(item)
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        self.pop()
    }

    fn peek(&self) -> Option<&Self::Elem> {
        self.peek()
    }
}
