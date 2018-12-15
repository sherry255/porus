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
