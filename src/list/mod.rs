pub trait List {
    type Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem>;
}

pub trait ListMut: List {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

pub fn get<T: List>(list: &T, index: usize) -> &T::Elem {
    List::get(list, index).unwrap()
}

pub fn get_mut<T: ListMut>(list: &mut T, index: usize) -> &mut T::Elem {
    ListMut::get_mut(list, index).unwrap()
}

#[macro_use]
pub mod slice;

mod iter;
pub use self::iter::iter;

pub mod sort;
pub use self::sort::{
    bubble_sort, insertion_sort, is_stable_sort, quick_sort, selection_sort, shell_sort,
};
