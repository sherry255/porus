use crate::collection::Collection;
use crate::list::List;

pub struct StaticArray<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> StaticArray<'a, T> {
    pub const fn new(slice: &'a [T]) -> Self {
        StaticArray { slice }
    }
}

impl<'a, T: 'a> Collection for StaticArray<'a, T> {
    fn size(&self) -> usize {
        self.slice.len()
    }
}

impl<'a, T: 'a> List for StaticArray<'a, T> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.slice.get(index)
    }
}

pub macro static_array($($arg:tt)*) {
    &StaticArray::new(&[$($arg)*])
}
