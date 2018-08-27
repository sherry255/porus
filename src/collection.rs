pub trait Collection {
    fn size(&self) -> usize;
}

pub fn size<T: Collection>(c: &T) -> usize {
    Collection::size(c)
}
