use crate::collection::{self, Collection};
use crate::heap::Heap;
use crate::list::{self, ListMut};
use crate::stack::{self, Stack};

pub fn parent_index(d: usize, index: usize) -> Option<usize> {
    usize::checked_sub(index, 1).map(|i| usize::wrapping_div(i, d))
}

pub fn child_index(d: usize, index: usize, n: usize) -> usize {
    usize::saturating_add(usize::saturating_mul(d, index), usize::saturating_add(1, n))
}

pub fn siftup<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(
    d: usize,
    l: &mut L,
    n: usize,
    gt: F,
) {
    if let Some(parent) = parent_index(d, n) {
        if !gt(list::get(l, n), list::get(l, parent)) {
            return;
        }
        list::swap(l, n, parent);
        siftup(d, l, parent, gt);
    }
}

pub fn siftdown<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(
    d: usize,
    l: &mut L,
    n: usize,
    gt: F,
) {
    let largest = (child_index(d, n, 0)..Ord::min(collection::size(l), child_index(d, n, d))).fold(
        n,
        |largest, c| {
            if gt(list::get(l, c), list::get(l, largest)) {
                c
            } else {
                largest
            }
        },
    );

    if largest > n {
        list::swap(l, n, largest);
        siftdown(d, l, largest, gt);
    }
}

pub fn heapify<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(d: usize, l: &mut L, gt: F) {
    if let Some(index) = usize::checked_sub(collection::size(l), 1) {
        if let Some(parent) = parent_index(d, index) {
            let mut n = parent;
            loop {
                siftdown(d, l, n, &gt);
                if let Some(n1) = usize::checked_sub(n, 1) {
                    n = n1;
                } else {
                    break;
                }
            }
        }
    }
}

pub struct DHeap<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool> {
    d: usize,
    list: L,
    gt: F,
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> DHeap<E, L, F> {
    pub fn new(d: usize, list: L, gt: F) -> Self {
        Self { d, list, gt }
    }
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> Collection
    for DHeap<E, L, F>
{
    fn size(&self) -> usize {
        collection::size(&self.list)
    }
}

impl<E, L: ListMut<Elem = E> + Stack<Elem = E>, F: Fn(&E, &E) -> bool> Heap for DHeap<E, L, F> {
    type Elem = E;

    fn push(&mut self, item: E) {
        let size = collection::size(self);
        stack::push(&mut self.list, item);
        siftup(self.d, &mut self.list, size, &self.gt)
    }

    fn pop(&mut self) -> Option<E> {
        match usize::checked_sub(collection::size(self), 1) {
            None => None,
            Some(index) => {
                list::swap(&mut self.list, 0, index);
                let result = stack::pop(&mut self.list);
                siftdown(self.d, &mut self.list, 0, &self.gt);
                Some(result)
            }
        }
    }

    fn peek(&self) -> Option<&E> {
        if collection::size(self) == 0 {
            None
        } else {
            Some(list::get(&self.list, 0))
        }
    }
}
