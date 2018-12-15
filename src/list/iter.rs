use super::List;
use crate::collection;
use crate::stream::{Cloned, DoubleEndedStream, ExactSizeStream, Stream};

pub struct ListStream<'a, T: 'a + List>
where
    T::Elem: Clone,
{
    list: &'a T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + List> Stream for ListStream<'a, T>
where
    T::Elem: Clone,
{
    type Item = T::Elem;

    fn next(&mut self) -> Option<&Self::Item> {
        if self.start < self.end {
            let index = self.start;
            self.start += 1;
            List::get(self.list, index)
        } else {
            None
        }
    }
}

impl<'a, T: 'a + List> ExactSizeStream for ListStream<'a, T>
where
    T::Elem: Clone,
{
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a, T: 'a + List> DoubleEndedStream for ListStream<'a, T>
where
    T::Elem: Clone,
{
    fn next_back(&mut self) -> Option<&Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            let index = self.end;
            List::get(self.list, index)
        } else {
            None
        }
    }
}

pub fn iter<T: List>(list: &T) -> Cloned<T::Elem, ListStream<T>>
where
    T::Elem: Clone,
{
    Stream::cloned(ListStream {
        list,
        start: 0,
        end: collection::size(list),
    })
}
