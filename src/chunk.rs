use core::marker::PhantomData;
use core::num::NonZeroUsize;
use porus::alloc::{allocate, deallocate, reallocate, Allocator};
use porus::capacity::{CapacityPolicy, DefaultCapacityPolicy};
use porus::os::OSAllocator;
use porus::pool::{self, Pool};
use porus::ptr::{get, get_mut, read, write};

#[derive(Clone, Copy)]
pub struct Handle(NonZeroUsize);

impl pool::Handle for Handle {}

#[allow(unions_with_drop_fields)]
union Node<T> {
    data: T,
    next: Option<NonZeroUsize>,
}

pub struct Chunk<T, P: CapacityPolicy = DefaultCapacityPolicy, A: Allocator = OSAllocator> {
    size: usize,
    capacity: usize,
    next: Option<NonZeroUsize>,
    data: *mut Node<T>,
    allocator: A,
    _policy: PhantomData<P>,
}

impl<T, P: CapacityPolicy, A: Allocator> Chunk<T, P, A> {
    pub fn new(mut allocator: A, capacity: usize) -> Self {
        let capacity = P::initial(capacity);
        let data = unsafe { allocate(&mut allocator, capacity) };
        Chunk {
            size: 0,
            capacity,
            next: None,
            data,
            allocator,
            _policy: PhantomData,
        }
    }
}

impl<T, P: CapacityPolicy, A: Allocator + Default> Chunk<T, P, A> {
    pub fn new_with_capacity(capacity: usize) -> Self {
        Chunk::new(Default::default(), capacity)
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Drop for Chunk<T, P, A> {
    fn drop(&mut self) {
        unsafe { deallocate(&mut self.allocator, self.data) };
    }
}

impl<T, P: CapacityPolicy, A: Allocator> Pool<T> for Chunk<T, P, A> {
    type Handle = Handle;

    fn get(&self, handle: Handle) -> &T {
        unsafe { &get(self.data, !handle.0.get()).data }
    }

    fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe { &mut get_mut(self.data, !handle.0.get()).data }
    }

    fn add(&mut self, item: T) -> Handle {
        let index = match self.next {
            None => {
                let size = self.size;
                self.size += 1;
                if size == self.capacity {
                    self.capacity = P::grow(size);
                    self.data =
                        unsafe { reallocate(&mut self.allocator, self.data, self.capacity) };
                }
                size
            }
            Some(handle) => {
                self.next = unsafe { get(self.data, !handle.get()).next };
                !handle.get()
            }
        };

        unsafe { write(self.data, index, Node { data: item }) };
        Handle(NonZeroUsize::new(!index).unwrap())
    }

    fn remove(&mut self, handle: Handle) -> T {
        let index = !handle.0.get();
        let node = unsafe { read(self.data, index) };
        unsafe { write(self.data, index, Node { next: self.next }) };
        self.next = Some(handle.0);
        unsafe { node.data }
    }
}

#[cfg(test)]
mod tests {
    use super::Handle;
    use core::mem::size_of;

    #[test]
    fn test_handle_size() {
        assert!(size_of::<Handle>() == size_of::<Option<Handle>>());
    }
}
