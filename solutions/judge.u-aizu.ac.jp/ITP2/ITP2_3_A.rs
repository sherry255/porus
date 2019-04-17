#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: isize = read!();
    let b: isize = read!();
    let c: isize = read!();
    writelnf!(
        "{:d} {:d}",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(a, b), c)
    );
}
