#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: usize = read!();
    let b: usize = read!();
    writelnf!("{:d} {:d}", a * b, (a + b) * 2);
}
