use super::{get, List};
use crate::collection;
use core::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};

pub struct ListIterator<'a, T: 'a + List>
where
    T::Elem: Clone,
{
    list: &'a T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + List> Iterator for ListIterator<'a, T>
where
    T::Elem: Clone,
{
    type Item = T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let index = self.start;
            self.start = usize::wrapping_add(self.start, 1);
            Some(Clone::clone(get(self.list, index)))
        } else {
            None
        }
    }
}

impl<'a, T: 'a + List> ExactSizeIterator for ListIterator<'a, T>
where
    T::Elem: Clone,
{
    fn len(&self) -> usize {
        usize::saturating_sub(self.end, self.start)
    }
}

impl<'a, T: 'a + List> DoubleEndedIterator for ListIterator<'a, T>
where
    T::Elem: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end = usize::wrapping_sub(self.end, 1);
            let index = self.end;
            Some(Clone::clone(get(self.list, index)))
        } else {
            None
        }
    }
}

pub fn iter<T: List>(list: &T) -> ListIterator<T>
where
    T::Elem: Clone,
{
    ListIterator {
        list,
        start: 0,
        end: collection::size(list),
    }
}
