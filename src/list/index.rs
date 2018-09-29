use super::{List, ListMut};
use core::mem;
use core::ops::{Bound, RangeBounds};
use crate::collection::{self, Collection};

pub fn get<L: List>(list: &L, index: usize) -> &<L as List>::Elem {
    List::get(list, index).unwrap()
}

pub fn get_mut<L: ListMut>(list: &mut L, index: usize) -> &mut <L as List>::Elem {
    ListMut::get_mut(list, index).unwrap()
}

pub fn swap<L: ListMut>(list: &mut L, i: usize, j: usize) {
    if i == j {
        return;
    }

    let mut t = unsafe { mem::uninitialized() };
    mem::swap(&mut t, get_mut(list, i));
    mem::swap(&mut t, get_mut(list, j));
    mem::swap(&mut t, get_mut(list, i));
    mem::forget(t);
}

pub fn start_bound<T: RangeBounds<usize>>(bound: &T) -> usize {
    match RangeBounds::start_bound(bound) {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x - 1,
    }
}

pub fn end_bound<T: RangeBounds<usize>>(bound: &T, default: usize) -> usize {
    match RangeBounds::end_bound(bound) {
        Bound::Unbounded => default,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    }
}

pub trait Slice<L: List> {
    fn slice<'a, T: RangeBounds<usize>>(&'a self, bound: &T) -> ListView<'a, L>;
}

pub trait SliceMut<L: ListMut> {
    fn slice_mut<'a, T: RangeBounds<usize>>(&'a mut self, bound: &T) -> ListMutView<'a, L>;
}

pub fn slice<'a, L: List, S: 'a + Slice<L>, T: RangeBounds<usize>>(
    list: &'a S,
    bound: T,
) -> ListView<'a, L> {
    Slice::slice(list, &bound)
}

pub fn slice_mut<'a, L: ListMut, S: 'a + SliceMut<L>, T: RangeBounds<usize>>(
    list: &'a mut S,
    bound: T,
) -> ListMutView<'a, L> {
    SliceMut::slice_mut(list, &bound)
}

pub struct ListView<'a, L: 'a + List> {
    list: &'a L,
    start: usize,
    size: usize,
}

pub struct ListMutView<'a, L: 'a + ListMut> {
    list: &'a mut L,
    start: usize,
    size: usize,
}

impl<'a, L: 'a + List> Slice<L> for ListView<'a, L> {
    fn slice<'b, T: RangeBounds<usize>>(&'b self, bound: &T) -> ListView<'b, L> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ListView {
            list: self.list,
            start: self.start + start,
            size: end - start,
        }
    }
}

impl<L: List> Slice<L> for L {
    fn slice<'a, T: RangeBounds<usize>>(&'a self, bound: &T) -> ListView<'a, Self> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ListView {
            list: self,
            start,
            size: end - start,
        }
    }
}

impl<'a, L: 'a + ListMut> SliceMut<L> for ListMutView<'a, L> {
    fn slice_mut<'b, T: RangeBounds<usize>>(&'b mut self, bound: &T) -> ListMutView<'b, L> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ListMutView {
            list: self.list,
            start: self.start + start,
            size: end - start,
        }
    }
}

impl<L: ListMut> SliceMut<L> for L {
    fn slice_mut<'a, T: RangeBounds<usize>>(&'a mut self, bound: &T) -> ListMutView<'a, Self> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ListMutView {
            list: self,
            start,
            size: end - start,
        }
    }
}

impl<'a, L: 'a + List> Collection for ListView<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + ListMut> Collection for ListMutView<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + List> List for ListView<'a, L> {
    type Elem = <L as List>::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            Some(List::get(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}

impl<'a, L: 'a + ListMut> List for ListMutView<'a, L> {
    type Elem = <L as List>::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            Some(List::get(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}

impl<'a, L: 'a + ListMut> ListMut for ListMutView<'a, L> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem> {
        if index < self.size {
            Some(ListMut::get_mut(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}
