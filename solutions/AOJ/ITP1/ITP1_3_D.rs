#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let a: usize = read!();
    let b: usize = read!();
    let c: usize = read!();
    writelnf!("{:d}", (a..=b).filter(|x| (&c) % x == 0).count());
}
