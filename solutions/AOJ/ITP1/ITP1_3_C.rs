#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let x: usize = read!();
        let y: usize = read!();
        if (x == 0) && (y == 0) {
            break;
        }
        writelnf!("{:d} {:d}", Ord::min(x, y), Ord::max(x, y));
    }
}
