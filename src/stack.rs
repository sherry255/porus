pub trait Stack {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push(&mut self, elem: Self::Elem);
    fn pop(&mut self) -> Option<Self::Elem>;
    fn top(&self) -> Option<&Self::Elem>;
}

pub fn is_empty<T: Stack>(s: &T) -> bool {
    Stack::is_empty(s)
}

pub fn push<T: Stack>(s: &mut T, elem: T::Elem) {
    Stack::push(s, elem)
}

pub fn pop<T: Stack>(s: &mut T) -> T::Elem {
    Stack::pop(s).unwrap()
}

pub fn top<T: Stack>(s: &T) -> &T::Elem {
    Stack::top(s).unwrap()
}


use alloc::vec::Vec;

impl<T> Stack for Vec<T> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn push(&mut self, elem: Self::Elem) {
        self.push(elem)
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        self.pop()
    }

    fn top(&self) -> Option<&Self::Elem> {
        if self.is_empty() {
            None
        } else {
            Some(&self[self.len() - 1])
        }
    }
}
