use crate::collection::Collection;

pub trait List: Collection {
    type Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem>;
}

pub trait ListMut: List {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

mod index;
pub use self::index::{get, get_mut, swap};

mod slice;
pub use self::slice::{slice, slice_mut};

mod iter;
pub use self::iter::iter;

pub mod sort;
pub use self::sort::{
    bubble_sort, insertion_sort, is_stable_sort, quick_sort, selection_sort, shell_sort,
};
