pub trait CapacityPolicy {
    fn initial(capacity: usize) -> usize;
    fn grow(capacity: usize) -> usize;
    fn shrink(size: usize, capacity: usize) -> usize;
}

pub struct FixedCapacityPolicy {}

impl CapacityPolicy for FixedCapacityPolicy {
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

pub struct DefaultCapacityPolicy {}

impl CapacityPolicy for DefaultCapacityPolicy {
    fn initial(size: usize) -> usize {
        if size < 10 {
            10
        } else {
            size
        }
    }

    fn grow(capacity: usize) -> usize {
        capacity + (capacity / 2)
    }

    fn shrink(size: usize, capacity: usize) -> usize {
        let new_capacity = if size * 9 / 4 < capacity {
            size * 3 / 2
        } else {
            capacity
        };
        if new_capacity < 10 {
            10
        } else {
            new_capacity
        }
    }
}
