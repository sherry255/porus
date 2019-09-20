use crate::collection::{self, Collection};
use crate::heap::Heap;
use crate::list::{self, ListMut};
use crate::stack::{self, Stack};

pub const fn parent_index(d: usize, index: usize) -> usize {
    (index - 1) / d
}

pub const fn child_index(d: usize, index: usize, n: usize) -> usize {
    d * index + 1 + n
}

pub fn siftup<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(
    d: usize,
    l: &mut L,
    n: usize,
    gt: F,
) {
    if n > 0 {
        let parent = parent_index(d, n);
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
    let size = collection::size(l);

    if size > 1 {
        let mut n = parent_index(d, size - 1);
        loop {
            siftdown(d, l, n, &gt);
            if n == 0 {
                break;
            }
            n -= 1;
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
        let size = collection::size(self);
        if size == 0 {
            None
        } else {
            list::swap(&mut self.list, 0, size - 1);
            let result = stack::pop(&mut self.list);
            siftdown(self.d, &mut self.list, 0, &self.gt);
            Some(result)
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
