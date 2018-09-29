#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<usize>::new_from_iter((0..n).map(|_| read!()));

    let pivot = sorting::partition(a, &PartialOrd::le);

    let l = &list::slice(a, ..pivot);
    let r = &list::slice(a, (pivot + 1)..);

    writelnf!(
        "{}[{:d}]{}",
        join(f!(""), list::iter(l).map(|e| f!("{:d} ", e))),
        list::get(a, pivot),
        join(f!(""), list::iter(r).map(|e| f!(" {:d}", e)))
    );
}
