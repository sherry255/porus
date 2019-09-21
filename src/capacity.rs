pub trait Policy {
    fn initial(capacity: usize) -> usize;
    fn grow(capacity: usize) -> usize;
    fn shrink(size: usize, capacity: usize) -> usize;
}

pub struct FixedPolicy {}

impl Policy for FixedPolicy {
    fn initial(size: usize) -> usize {
        size
    }

    fn grow(capacity: usize) -> usize {
        capacity
    }

    fn shrink(_size: usize, capacity: usize) -> usize {
        capacity
    }
}

pub struct DefaultPolicy {}

impl Policy for DefaultPolicy {
    fn initial(size: usize) -> usize {
        Ord::max(10, size)
    }

    fn grow(capacity: usize) -> usize {
        usize::saturating_add(capacity, capacity >> 1)
    }

    fn shrink(size: usize, capacity: usize) -> usize {
        let g = Self::grow(size);
        let new_capacity = if Self::grow(g) < capacity {
            g
        } else {
            capacity
        };
        Self::initial(new_capacity)
    }
}
