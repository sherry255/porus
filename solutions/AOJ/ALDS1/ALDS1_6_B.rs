#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n: isize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));

    let pivot = list::sort::partition(a, &PartialOrd::le);

    let l = slice!(a, [, pivot as isize]);
    let r = slice!(a, [(pivot + 1) as isize,]);

    writelnf!(
        "{}[{:d}]{}",
        join(f!(""), list::iter(l).map(|e| f!("{e:d} "))),
        *list::get(a, pivot),
        join(f!(""), list::iter(r).map(|e| f!(" {e:d}")))
    );
}
