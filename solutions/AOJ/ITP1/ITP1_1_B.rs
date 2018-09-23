#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let x: usize = read!();
    writelnf!("{:d}", x * x * x);
}
