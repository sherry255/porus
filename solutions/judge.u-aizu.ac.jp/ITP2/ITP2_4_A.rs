#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));
    let q: usize = read!();

    for _ in 0..q {
        let b: usize = read!();
        let e: usize = read!();

        list::reverse(&mut list::slice_mut(a, b..e));
    }

    writelnf!("{}", join(f!(" "), list::iter(a).map(|e| f!("{:d}", e))));
}
