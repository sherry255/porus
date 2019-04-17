#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let t: isize = read!();
    let s = t % 60;
    let mut m = t / 60;
    let h = m / 60;
    m = m % 60;
    writelnf!("{:d}:{:d}:{:d}", h, m, s);
}
