#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: usize = read!();
    let b: usize = read!();
    writelnf!("{:d} {:d} {:.6f}", a / b, a % b, (a as f64) / (b as f64));
}
