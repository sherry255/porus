use crate::allocator;
use crate::pool::{self, Handle, Pool};
use crate::stack::Stack;
use core::marker::PhantomData;

pub struct Node<H: Handle, T> {
    next: Option<H>,
    data: T,
}

pub struct SinglyLinkedList<
    T,
    H: Handle = allocator::Handle,
    P: Pool<Elem = Node<H, T>, Handle = H> = allocator::Pool<Node<H, T>>,
> {
    pool: P,
    sentinel: Option<H>,
    _data: PhantomData<T>,
}

impl<T, H: Handle, P: Pool<Elem = Node<H, T>, Handle = H>> SinglyLinkedList<T, H, P> {
    pub fn new_with_pool(pool: P) -> Self {
        Self {
            pool,
            sentinel: None,
            _data: PhantomData,
        }
    }
}

impl<T, H: Handle, P: Pool<Elem = Node<H, T>, Handle = H> + Default> SinglyLinkedList<T, H, P> {
    pub fn new() -> Self {
        Self::new_with_pool(Default::default())
    }
}

impl<T, H: Handle, P: Pool<Elem = Node<H, T>, Handle = H> + Default> Default
    for SinglyLinkedList<T, H, P>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, H: Handle, P: Pool<Elem = Node<H, T>, Handle = H>> Stack for SinglyLinkedList<T, H, P> {
    type Elem = T;

    fn is_empty(&self) -> bool {
        self.sentinel.is_none()
    }

    fn push(&mut self, elem: T) {
        let node = Node {
            next: self.sentinel,
            data: elem,
        };
        let handle = pool::add(&mut self.pool, node);
        self.sentinel = Some(handle);
    }

    fn pop(&mut self) -> Option<T> {
        match self.sentinel {
            None => None,
            Some(handle) => {
                let node = pool::remove(&mut self.pool, handle);
                self.sentinel = node.next;
                Some(node.data)
            }
        }
    }
}
