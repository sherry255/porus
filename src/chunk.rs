use core::num::NonZeroUsize;
use porus::alloc::Allocator;
use porus::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use porus::excess::Excess;
use porus::os;
use porus::pool::{self, Pool};

#[derive(Clone, Copy)]
pub struct Handle(NonZeroUsize);

impl pool::Handle for Handle {}

#[allow(unions_with_drop_fields)]
union Node<T> {
    data: T,
    next: Option<NonZeroUsize>,
}

pub struct Chunk<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = os::Allocator> {
    size: usize,
    next: Option<NonZeroUsize>,
    data: Excess<Node<T>, P, A>,
}

impl<T, P: CapacityPolicy, A: Allocator> Chunk<T, P, A> {
    pub fn new(allocator: A, capacity: usize) -> Self {
        Chunk {
            size: 0,
            next: None,
            data: Excess::new(allocator, capacity),
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Chunk<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Chunk::new(Default::default(), capacity)
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Pool for Chunk<T, P, A> {
    type Handle = Handle;
    type Elem = T;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &self.data.get(!handle.0.get()).data }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut self.data.get_mut(!handle.0.get()).data }
    }

    fn add(&mut self, item: T) -> Handle {
        let index = match self.next {
            None => {
                let size = self.size;
                self.size += 1;
                if size == self.data.capacity() {
                    assert!(self.data.grow(0) > 0);
                }
                size
            }
            Some(handle) => {
                self.next = unsafe { self.data.get(!handle.get()).next };
                !handle.get()
            }
        };

        self.data.write(index, Node { data: item });
        Handle(NonZeroUsize::new(!index).unwrap())
    }

    fn remove(&mut self, handle: Handle) -> T {
        let index = !handle.0.get();
        let node = self.data.read(index);
        self.data.write(index, Node { next: self.next });
        self.next = Some(handle.0);
        unsafe { node.data }
    }
}
