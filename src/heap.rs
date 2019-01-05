pub trait Heap {
    type Elem;

    fn insert(&mut self, item: Self::Elem);
    fn extract(&mut self) -> Option<Self::Elem>;
    fn top(&self) -> Option<&Self::Elem>;
}

pub fn insert<H: Heap>(heap: &mut H, item: H::Elem) {
    Heap::insert(heap, item);
}

pub fn extract<H: Heap>(heap: &mut H) -> H::Elem {
    Heap::extract(heap).unwrap()
}

pub fn top<H: Heap>(heap: &mut H) -> &H::Elem {
    Heap::top(heap).unwrap()
}
