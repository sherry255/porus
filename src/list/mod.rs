use crate::collection::{self, Collection};
use core::ops::{Bound, RangeBounds};

pub trait List: Collection {
    type Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem>;
}

#[allow(clippy::module_name_repetitions)]
pub trait ListMut: List {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

pub fn get<L: List>(list: &L, index: usize) -> &<L as List>::Elem {
    List::get(list, index).unwrap()
}

pub fn get_mut<L: ListMut>(list: &mut L, index: usize) -> &mut <L as List>::Elem {
    ListMut::get_mut(list, index).unwrap()
}

pub fn set<L: ListMut>(list: &mut L, index: usize, elem: <L as List>::Elem) {
    *get_mut(list, index) = elem
}

pub fn swap<L: ListMut>(list: &mut L, i: usize, j: usize) {
    if i == j {
        return;
    }

    let mut t = unsafe { core::mem::uninitialized() };
    core::mem::swap(&mut t, get_mut(list, i));
    core::mem::swap(&mut t, get_mut(list, j));
    core::mem::swap(&mut t, get_mut(list, i));
    core::mem::forget(t);
}

pub fn reverse<L: ListMut>(list: &mut L) {
    let mut l = 0;
    let mut r = collection::size(list);
    while l < r {
        swap(list, l, r - 1);
        l += 1;
        r -= 1;
    }
}

pub fn rotate_left<L: ListMut>(list: &mut L, n: usize) {
    let size = collection::size(list);
    if size == 0 {
        return;
    }

    let m = size - n % size;

    reverse(&mut slice_mut(list, ..m));
    reverse(&mut slice_mut(list, m..));
    reverse(list);
}

pub fn rotate_right<L: ListMut>(list: &mut L, n: usize) {
    let size = collection::size(list);
    if size == 0 {
        return;
    }

    let m = n % size;

    reverse(&mut slice_mut(list, ..m));
    reverse(&mut slice_mut(list, m..));
    reverse(list);
}

pub struct View<'a, L: 'a + List> {
    list: &'a L,
    start: usize,
    size: usize,
}

pub struct ViewMut<'a, L: 'a + ListMut> {
    list: &'a mut L,
    start: usize,
    size: usize,
}

pub trait Slice<L: List> {
    fn slice<'a, T: RangeBounds<usize>>(&'a self, bound: &T) -> View<'a, L>;
}

pub trait SliceMut<L: ListMut> {
    fn slice_mut<'a, T: RangeBounds<usize>>(&'a mut self, bound: &T) -> ViewMut<'a, L>;
}

pub fn slice<'a, L: List, S: 'a + Slice<L>, T: RangeBounds<usize>>(
    list: &'a S,
    bound: T,
) -> View<'a, L> {
    Slice::slice(list, &bound)
}

pub fn slice_mut<'a, L: ListMut, S: 'a + SliceMut<L>, T: RangeBounds<usize>>(
    list: &'a mut S,
    bound: T,
) -> ViewMut<'a, L> {
    SliceMut::slice_mut(list, &bound)
}

fn start_bound<T: RangeBounds<usize>>(bound: &T) -> usize {
    match RangeBounds::start_bound(bound) {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x - 1,
    }
}

fn end_bound<T: RangeBounds<usize>>(bound: &T, default: usize) -> usize {
    match RangeBounds::end_bound(bound) {
        Bound::Unbounded => default,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    }
}

impl<'a, L: 'a + List> Slice<L> for View<'a, L> {
    fn slice<'b, T: RangeBounds<usize>>(&'b self, bound: &T) -> View<'b, L> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        View {
            list: self.list,
            start: self.start + start,
            size: end - start,
        }
    }
}

impl<L: List> Slice<L> for L {
    fn slice<'a, T: RangeBounds<usize>>(&'a self, bound: &T) -> View<'a, Self> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        View {
            list: self,
            start,
            size: end - start,
        }
    }
}

impl<'a, L: 'a + ListMut> SliceMut<L> for ViewMut<'a, L> {
    fn slice_mut<'b, T: RangeBounds<usize>>(&'b mut self, bound: &T) -> ViewMut<'b, L> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ViewMut {
            list: self.list,
            start: self.start + start,
            size: end - start,
        }
    }
}

impl<L: ListMut> SliceMut<L> for L {
    fn slice_mut<'a, T: RangeBounds<usize>>(&'a mut self, bound: &T) -> ViewMut<'a, Self> {
        let start = start_bound(bound);
        let end = end_bound(bound, collection::size(self));

        ViewMut {
            list: self,
            start,
            size: end - start,
        }
    }
}

impl<'a, L: 'a + List> Collection for View<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + ListMut> Collection for ViewMut<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + List> List for View<'a, L> {
    type Elem = <L as List>::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            Some(List::get(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}

impl<'a, L: 'a + ListMut> List for ViewMut<'a, L> {
    type Elem = <L as List>::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            Some(List::get(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}

impl<'a, L: 'a + ListMut> ListMut for ViewMut<'a, L> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem> {
        if index < self.size {
            Some(ListMut::get_mut(self.list, self.start + index).unwrap())
        } else {
            None
        }
    }
}

use alloc::vec::Vec;

impl<T> List for Vec<T> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
}

impl<T> ListMut for Vec<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

mod iter;
pub use self::iter::iter;

pub mod sorting;
pub use self::sorting::{
    bubble_sort, insertion_sort, is_stable_sort, quick_sort, selection_sort, shell_sort,
};

mod search;
pub use self::search::{bsearch, find};
