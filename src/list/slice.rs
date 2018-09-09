use super::super::collection::Collection;
use super::super::range::Range;
use super::{List, ListMut};

fn slice(size: isize, range: &Range) -> (isize, isize, isize) {
    let start = range.start(size);
    let stop = range.stop(size);
    let step = range.step();

    if step > 0 {
        if !((start >= 0) && (start <= size)) {
            panic!("start must in [0,size]");
        }

        if !((stop >= 0) && (stop <= size)) {
            panic!("stop must in [0,size]");
        }

        (
            start,
            if stop <= start {
                0
            } else {
                (stop - start - 1) / step + 1
            },
            step,
        )
    } else if step < 0 {
        if !((start >= -1) && (start < size)) {
            panic!("start must in [-1,size)");
        }

        if !((stop >= -1) && (stop < size)) {
            panic!("stop must in [-1,size)");
        }

        (
            start,
            if stop >= start {
                0
            } else {
                (stop - start + 1) / step + 1
            },
            step,
        )
    } else {
        panic!("step must not be 0");
    }
}

fn slice_index(base: usize, index: usize, step: isize) -> usize {
    ((base as isize) + step * (index as isize)) as usize
}

pub struct ListView<'a, T: 'a + List> {
    list: &'a T,
    offset: usize,
    size: usize,
    step: isize,
}

impl<'a, T: List> Collection for ListView<'a, T> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, T: List> List for ListView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            List::get(self.list, slice_index(self.offset, index, self.step))
        } else {
            None
        }
    }
}

pub trait Slice<'a, T: List + Collection> {
    fn new(&'a self, range: &Range) -> ListView<'a, T>;
}

impl<'a, 'b: 'a, T: List + Collection> Slice<'b, T> for ListView<'a, T> {
    fn new(&'b self, range: &Range) -> ListView<'b, T> {
        let (offset, size, step) = slice(Collection::size(self) as isize, range);

        ListView {
            list: self.list,
            offset: slice_index(self.offset, offset as usize, self.step),
            size: size as usize,
            step: self.step * step,
        }
    }
}

impl<'a, T: List + Collection> Slice<'a, T> for T {
    fn new(&'a self, range: &Range) -> ListView<'a, T> {
        let (offset, size, step) = slice(Collection::size(self) as isize, range);
        ListView {
            list: self,
            offset: offset as usize,
            size: size as usize,
            step,
        }
    }
}

#[macro_export]
macro_rules! slice {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &$crate::list::slice::Slice::new($list, range!($($arg)+))
    }
}

pub struct ListMutView<'a, T: 'a + ListMut> {
    list: &'a mut T,
    offset: usize,
    size: usize,
    step: isize,
}

impl<'a, T: ListMut> Collection for ListMutView<'a, T> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, T: ListMut> List for ListMutView<'a, T> {
    type Elem = T::Elem;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        if index < self.size {
            List::get(self.list, slice_index(self.offset, index, self.step))
        } else {
            None
        }
    }
}

impl<'a, T: ListMut> ListMut for ListMutView<'a, T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem> {
        if index < self.size {
            ListMut::get_mut(self.list, slice_index(self.offset, index, self.step))
        } else {
            None
        }
    }
}

pub trait SliceMut<'a, T: ListMut + Collection> {
    fn new(&'a mut self, range: &Range) -> ListMutView<'a, T>;
}

impl<'a, 'b: 'a, T: ListMut + Collection> SliceMut<'b, T> for ListMutView<'a, T> {
    fn new(&'b mut self, range: &Range) -> ListMutView<'b, T> {
        let (offset, size, step) = slice(Collection::size(self) as isize, range);

        ListMutView {
            list: self.list,
            offset: slice_index(self.offset, offset as usize, self.step),
            size: size as usize,
            step: self.step * step,
        }
    }
}

impl<'a, T: ListMut + Collection> SliceMut<'a, T> for T {
    fn new(&'a mut self, range: &Range) -> ListMutView<'a, T> {
        let (offset, size, step) = slice(Collection::size(self) as isize, range);
        ListMutView {
            list: self,
            offset: offset as usize,
            size: size as usize,
            step,
        }
    }
}

#[macro_export]
macro_rules! slice_mut {
    ($list:expr, [ $($arg:tt)+ ]) => {
        &mut $crate::list::slice::SliceMut::new($list, range!($($arg)+))
    }
}
