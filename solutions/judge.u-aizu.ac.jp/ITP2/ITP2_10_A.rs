#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let x: u32 = read!();
    writelnf!("{:032b}", x);
    writelnf!("{:032b}", !x);
    writelnf!("{:032b}", x << 1);
    writelnf!("{:032b}", x >> 1);
}
