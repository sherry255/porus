use crate::block::Block;
use crate::capacity::{DefaultPolicy, Policy};
use crate::pool::{self, Pool};
use alloc::alloc::{Alloc, Global};
use core::num::NonZeroUsize;
use core::mem::ManuallyDrop;

#[derive(Clone, Copy)]
struct Index(NonZeroUsize);

impl Index {
    pub fn new(x: usize) -> Self {
        Self(NonZeroUsize::new(!x).expect("index overflow"))
    }

    pub fn get(self) -> usize {
        !self.0.get()
    }
}

#[derive(Clone, Copy)]
pub struct Handle(Index);

impl pool::Handle for Handle {}

union Node<T> {
    data: ManuallyDrop<T>,
    next: Option<Index>,
}

pub struct Chunk<T, P: Policy = DefaultPolicy, A: Alloc = Global> {
    size: usize,
    next: Option<Index>,
    data: Block<Node<T>, P, A>,
}

impl<T, P: Policy, A: Alloc> Chunk<T, P, A> {
    pub fn new(allocator: A, capacity: usize) -> Self {
        Self {
            size: 0,
            next: None,
            data: Block::new(allocator, capacity),
        }
    }
}

impl<T, P: Policy, A: Alloc + Default> Chunk<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self::new(Default::default(), capacity)
    }
}

impl<T, P: Policy, A: Alloc> Pool for Chunk<T, P, A> {
    type Handle = Handle;
    type Elem = T;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &self.data.get(handle.0.get()).data }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut self.data.get_mut(handle.0.get()).data }
    }

    fn add(&mut self, item: T) -> Handle {
        let index = match self.next {
            None => {
                let size = self.size;
                self.size = usize::wrapping_add(self.size, 1);
                if size == self.data.capacity() {
                    assert!(self.data.grow(0) > 0);
                }
                size
            }
            Some(handle) => {
                self.next = unsafe { self.data.get(handle.get()).next };
                handle.get()
            }
        };

        self.data.write(index, Node { data: ManuallyDrop::new(item) });
        Handle(Index::new(index))
    }

    fn remove(&mut self, handle: Handle) -> T {
        let index = handle.0.get();
        let node = self.data.read(index);
        self.data.write(index, Node { next: self.next });
        self.next = Some(handle.0);
        ManuallyDrop::into_inner(unsafe { node.data })
    }
}
