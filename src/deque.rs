pub trait Deque {
    type Elem;

    fn is_empty(&self) -> bool;
    fn push_front(&mut self, elem: Self::Elem);
    fn pop_front(&mut self) -> Option<Self::Elem>;
    fn push_back(&mut self, elem: Self::Elem);
    fn pop_back(&mut self) -> Option<Self::Elem>;
    fn front(&self) -> Option<&Self::Elem>;
    fn back(&self) -> Option<&Self::Elem>;
}

pub fn is_empty<T: Deque>(q: &T) -> bool {
    Deque::is_empty(q)
}

pub fn push_front<T: Deque>(q: &mut T, elem: T::Elem) {
    Deque::push_front(q, elem)
}

pub fn pop_front<T: Deque>(q: &mut T) -> T::Elem {
    Deque::pop_front(q).unwrap()
}

pub fn push_back<T: Deque>(q: &mut T, elem: T::Elem) {
    Deque::push_back(q, elem)
}

pub fn pop_back<T: Deque>(q: &mut T) -> T::Elem {
    Deque::pop_back(q).unwrap()
}

pub fn front<T: Deque>(q: &T) -> &T::Elem {
    Deque::front(q).unwrap()
}

pub fn back<T: Deque>(q: &T) -> &T::Elem {
    Deque::back(q).unwrap()
}

pub struct Drain<'a, Q: Deque> {
    q: &'a mut Q,
}

impl<'a, Q: Deque> Iterator for Drain<'a, Q> {
    type Item = Q::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        self.q.pop_front()
    }
}

impl<'a, Q: Deque> DoubleEndedIterator for Drain<'a, Q> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.q.pop_back()
    }
}

pub fn drain<T: Deque>(q: &mut T) -> Drain<T> {
    Drain { q }
}

use alloc::collections::VecDeque;

impl<T> Deque for VecDeque<T> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn push_front(&mut self, elem: Self::Elem) {
        self.push_front(elem)
    }

    fn pop_front(&mut self) -> Option<Self::Elem> {
        self.pop_front()
    }

    fn push_back(&mut self, elem: Self::Elem) {
        self.push_back(elem)
    }

    fn pop_back(&mut self) -> Option<Self::Elem> {
        self.pop_back()
    }

    fn front(&self) -> Option<&Self::Elem> {
        self.front()
    }

    fn back(&self) -> Option<&Self::Elem> {
        self.back()
    }
}
