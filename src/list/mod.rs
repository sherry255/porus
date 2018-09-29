use crate::collection::Collection;

pub trait List: Collection {
    type Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem>;
}

#[allow(clippy::stutter)]
pub trait ListMut: List {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

mod index;
pub use self::index::{get, get_mut, slice, slice_mut, swap};

mod iter;
pub use self::iter::iter;

pub mod sorting;
pub use self::sorting::{
    bubble_sort, insertion_sort, is_stable_sort, quick_sort, selection_sort, shell_sort,
};
