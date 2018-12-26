use super::{get, List};
use crate::collection;

pub fn find<E: PartialEq, L: List<Elem = E>>(list: &L, elem: &E) -> Option<usize> {
    let size = collection::size(list);
    for i in 0..size {
        if get(list, i) == elem {
            return Some(i);
        }
    }
    None
}
