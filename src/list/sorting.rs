use super::ViewMut;
use super::{get, iter, slice_mut, swap, List, ListMut};
use crate::collection::{self, Collection};
use core::cmp::Ordering::{Greater, Less};

#[allow(clippy::nonminimal_bool)]
pub fn is_stable_sort<
    E,
    L: List<Elem = E> + Collection,
    F: Fn(&E, &E) -> bool,
    I: List<Elem = usize>,
>(
    list: &L,
    lt: F,
    index: &I,
) -> bool {
    iter(index).is_sorted_by(|i, j| {
        if !lt(get(list, *i), get(list, *j)) && !(i < j) {
            None
        } else {
            Some(Less)
        }
    })
}

pub fn bubble<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let mut count = 0;
    let size = collection::size(list);
    for (i, j) in (0..size).zip(1..size).rev() {
        if lt(get(list, j), get(list, i)) {
            swap(list, i, j);
            count = usize::wrapping_add(count, 1);
        }
    }
    count
}

pub fn bubble_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let size = collection::size(list);
    (0..size)
        .map(|i| bubble(&mut slice_mut(list, i..size), &lt))
        .sum()
}

pub fn bubble_sorted<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let mut count = 0;
    let size = collection::size(list);
    for (i, j) in (0..size).zip(1..size).rev() {
        if !lt(get(list, j), get(list, i)) {
            break;
        }
        swap(list, i, j);
        count = usize::wrapping_add(count, 1);
    }
    count
}

fn insertion_sort_g<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
    g: usize,
) -> usize {
    let mut count = 0;
    let size = collection::size(list);

    for i in g..size {
        let mut j = i;
        while (j >= g) && lt(get(list, j), get(list, usize::wrapping_sub(j, g))) {
            swap(list, j, usize::wrapping_sub(j, g));
            count = usize::wrapping_add(count, 1);
            j = usize::wrapping_sub(j, g);
        }
    }

    count
}

pub fn insertion_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let size = collection::size(list);
    (0..size)
        .map(|i| bubble_sorted(&mut slice_mut(list, 0..i), &lt))
        .sum()
}

pub fn shell_sort<
    E,
    L: ListMut<Elem = E> + Collection,
    F: Fn(&E, &E) -> bool,
    G: List<Elem = usize> + Collection,
>(
    list: &mut L,
    lt: F,
    gaps: &G,
) -> usize {
    let mut count = 0;
    for g in iter(gaps) {
        // for i in 0..g {
        //     count += insertion_sort(slice_mut!(list, [i,,g]), lt);
        // }
        count = usize::wrapping_add(count, insertion_sort_g(list, &lt, g));
    }
    count
}

pub fn selection_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let mut count = 0;
    let size = collection::size(list);
    for i in 0..size {
        if let Some(min) = (i..size).min_by(|x, y| {
            if lt(get(list, *y), get(list, *x)) {
                Greater
            } else {
                Less
            }
        }) {
            if min != i {
                swap(list, i, min);
                count = usize::wrapping_add(count, 1);
            }
        } else {
            unreachable!();
        }
    }
    count
}

#[allow(clippy::integer_arithmetic)]
pub fn partition<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) -> usize {
    let size = collection::size(list);
    let mut i = 0;
    for j in 0..size - 1 {
        if lt(get(list, j), get(list, size - 1)) {
            swap(list, j, i);
            i += 1;
        }
    }

    swap(list, i, size - 1);
    i
}

#[allow(clippy::integer_arithmetic)]
fn quick_sort_aux<'a, 'b: 'a, E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &'b mut ViewMut<'a, L>,
    lt: &F,
) {
    let size = collection::size(list);
    if size < 2 {
        return;
    }

    let p = partition(list, lt);
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, ..p), lt);
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, (p + 1)..), lt);
}

pub fn quick_sort<E, L: ListMut<Elem = E> + Collection, F: Fn(&E, &E) -> bool>(
    list: &mut L,
    lt: F,
) {
    quick_sort_aux::<E, L, F>(&mut slice_mut(list, 0..), &lt);
}
