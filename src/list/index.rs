use super::{List, ListMut};
use core::mem;

pub trait ListIndex<'a, L: 'a + List> {
    fn get(self, list: &'a L) -> &'a <L as List>::Elem;
}

pub trait ListMutIndex<'a, L: 'a + ListMut> {
    fn get_mut(self, list: &'a mut L) -> &'a mut <L as List>::Elem;
}

pub fn get<'a, L: 'a + List, I: ListIndex<'a, L>>(list: &'a L, index: I) -> &'a <L as List>::Elem {
    ListIndex::get(index, list)
}

pub fn get_mut<'a, L: 'a + ListMut, I: ListMutIndex<'a, L>>(
    list: &'a mut L,
    index: I,
) -> &'a mut <L as List>::Elem {
    ListMutIndex::get_mut(index, list)
}

impl<'a, L: 'a + List> ListIndex<'a, L> for usize {
    fn get(self, list: &'a L) -> &'a <L as List>::Elem {
        List::get(list, self).unwrap()
    }
}

impl<'a, 'b, L: 'a + List> ListIndex<'a, L> for &'b usize {
    fn get(self, list: &'a L) -> &'a <L as List>::Elem {
        List::get(list, *self).unwrap()
    }
}

impl<'a, L: 'a + ListMut> ListMutIndex<'a, L> for usize {
    fn get_mut(self, list: &'a mut L) -> &'a mut <L as List>::Elem {
        ListMut::get_mut(list, self).unwrap()
    }
}

impl<'a, 'b, L: 'a + ListMut> ListMutIndex<'a, L> for &'b usize {
    fn get_mut(self, list: &'a mut L) -> &'a mut <L as List>::Elem {
        ListMut::get_mut(list, *self).unwrap()
    }
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
