#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));

    let count = list::selection_sort(a, &PartialOrd::lt);

    writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{:d}", e))));
    writelnf!("{:d}", count);
}
