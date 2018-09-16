use super::List;
use core::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};
use porus::collection;

pub struct ListIter<'a, T: 'a + List>
where
    T::Elem: Copy,
{
    list: &'a T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + List> ExactSizeIterator for ListIter<'a, T>
where
    T::Elem: Copy,
{
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, T: 'a + List> Iterator for ListIter<'a, T>
where
    T::Elem: Copy,
{
    type Item = T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let index = self.start;
            self.start += 1;
            Some(*List::get(self.list, index).unwrap())
        } else {
            None
        }
    }
}

impl<'a, T: 'a + List> DoubleEndedIterator for ListIter<'a, T>
where
    T::Elem: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            let index = self.end;
            Some(*List::get(self.list, index).unwrap())
        } else {
            None
        }
    }
}

pub fn iter<T: List>(list: &T) -> ListIter<T>
where
    T::Elem: Copy,
{
    ListIter {
        list,
        start: 0,
        end: collection::size(list),
    }
}
