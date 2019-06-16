#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<usize> = &mut (0..n).map(|_| read!()).collect();
    writelnf!(
        "{}",
        join(f!(" "), list::iter(a).rev().map(|e| f!("{:d}", e)))
    );
}
