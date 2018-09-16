use core::marker::PhantomData;
use porus::alloc::Handle as OSHandle;
use porus::os::OSAllocator;
use porus::pool::{Handle, Pool};
use porus::stack::Stack;

pub struct Node<H: Handle, T> {
    next: Option<H>,
    data: T,
}

pub struct SinglyLinkedList<T, H: Handle = OSHandle, P: Pool<Node<H, T>, Handle = H> = OSAllocator>
{
    pool: P,
    sentinel: Option<H>,
    _data: PhantomData<T>,
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H> + Default> SinglyLinkedList<T, H, P> {
    pub fn new() -> Self {
        SinglyLinkedList {
            pool: Default::default(),
            sentinel: None,
            _data: PhantomData,
        }
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H> + Default> Default
    for SinglyLinkedList<T, H, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, H: Handle, P: Pool<Node<H, T>, Handle = H>> Stack for SinglyLinkedList<T, H, P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.sentinel.is_none()
    }

    fn push(&mut self, elem: T) {
        let node = Node {
            next: self.sentinel,
            data: elem,
        };
        let handle = Pool::add(&mut self.pool, node);
        self.sentinel = Some(handle);
    }

    fn pop(&mut self) -> Option<T> {
        match self.sentinel {
            None => None,
            Some(handle) => {
                let node = Pool::remove(&mut self.pool, handle);
                self.sentinel = node.next;
                Some(node.data)
            }
        }
    }
}
