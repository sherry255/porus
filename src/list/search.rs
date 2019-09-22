use super::{get, List};
use crate::collection;
use core::cmp::Ordering;

pub fn find<E: PartialEq, L: List<Elem = E>>(list: &L, elem: &E) -> Option<usize> {
    let size = collection::size(list);
    for i in 0..size {
        if get(list, i) == elem {
            return Some(i);
        }
    }
    None
}

const fn is_empty(r: &(usize, usize)) -> bool {
    r.0 >= r.1
}

const fn midpoint(r: &(usize, usize)) -> usize {
    usize::saturating_add(r.0, usize::saturating_sub(r.1, 1)) >> 1
}

const fn range_left(r: &(usize, usize), m: usize) -> (usize, usize) {
    (r.0, m)
}

const fn range_right(r: &(usize, usize), m: usize) -> (usize, usize) {
    (usize::saturating_add(m, 1), r.1)
}

pub fn bsearch<E: Ord, L: List<Elem = E>>(list: &L, elem: &E) -> (usize, usize) {
    let mut r = (0, collection::size(list));

    while !is_empty(&r) {
        let m = midpoint(&r);
        match Ord::cmp(elem, get(list, m)) {
            Ordering::Equal => break,
            Ordering::Less => r = range_left(&r, m),
            Ordering::Greater => r = range_right(&r, m),
        }
    }

    if is_empty(&r) {
        return r;
    }

    let mut rl = range_left(&r, midpoint(&r));
    let mut rr = range_right(&r, midpoint(&r));

    while !is_empty(&rl) {
        let m = midpoint(&rl);
        match Ord::cmp(elem, get(list, m)) {
            Ordering::Equal => rl = range_left(&rl, m),
            Ordering::Greater => rl = range_right(&rl, m),
            Ordering::Less => unreachable!(),
        }
    }

    while !is_empty(&rr) {
        let m = midpoint(&rr);
        match Ord::cmp(elem, get(list, m)) {
            Ordering::Equal => rr = range_right(&rr, m),
            Ordering::Less => rr = range_left(&rr, m),
            Ordering::Greater => unreachable!(),
        }
    }

    (rl.0, rr.1)
}
