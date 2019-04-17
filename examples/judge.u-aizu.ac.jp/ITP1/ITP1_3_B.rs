#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut i: usize = 1;
    loop {
        let x: usize = read!();
        if x == 0 {
            break;
        }
        writelnf!("Case {:d}: {:d}", i, x);
        i += 1;
    }
}
