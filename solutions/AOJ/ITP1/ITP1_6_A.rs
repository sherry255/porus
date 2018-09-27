#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<usize>::new_from_iter((0..n).map(|_| read!()));
    writelnf!(
        "{}",
        join(f!(" "), list::iter(a).rev().map(|e| f!("{:d}", e)))
    );
}
