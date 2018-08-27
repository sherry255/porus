use core::ops::{Index, IndexMut};

pub trait ListBase {
    type Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem>;
}

pub trait ListMutBase: ListBase {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

pub trait List: ListBase + Index<usize, Output = <Self as ListBase>::Elem> {}

pub trait ListMut: ListMutBase + IndexMut<usize, Output = <Self as ListBase>::Elem> {}

pub fn get<T: List>(list: &T, index: usize) -> Option<&T::Elem> {
    ListBase::get(list, index)
}

pub fn get_mut<T: ListMut>(list: &mut T, index: usize) -> Option<&mut T::Elem> {
    ListMutBase::get_mut(list, index)
}

#[macro_use]
pub mod slice;

mod iter;
pub use self::iter::{iter, iter_ref, iter_ref_mut};

pub mod sort;
pub use self::sort::{
    bubble_sort, insertion_sort, is_stable_sort, quick_sort, selection_sort, shell_sort,
};
