use super::List;
use core::iter::Iterator;

pub struct ListIter<'a, T: 'a + List>
where
    T::Elem: Copy,
{
    list: &'a T,
    index: usize,
}

impl<'a, T: 'a + List> Iterator for ListIter<'a, T>
where
    T::Elem: Copy,
{
    type Item = T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let it = List::get(self.list, index);
        self.index += 1;
        match it {
            None => None,
            Some(x) => Some(*x),
        }
    }
}

pub fn iter<T: List>(list: &T) -> ListIter<T>
where
    T::Elem: Copy,
{
    ListIter { list, index: 0 }
}
