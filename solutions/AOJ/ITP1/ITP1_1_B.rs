#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let x: usize = read!();
    writelnf!("{:d}", x * x * x);
}
