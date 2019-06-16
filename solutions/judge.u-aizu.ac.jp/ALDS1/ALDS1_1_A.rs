#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<isize> = &mut (0..n).map(|_| read!()).collect();

    writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{:d}", e))));

    for i in 2..n + 1 {
        sorting::bubble_sorted(&mut list::slice_mut(a, ..i), PartialOrd::lt);
        writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{:d}", e))));
    }
}
