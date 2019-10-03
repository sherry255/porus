#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: u32 = read!();
    let b: u32 = read!();
    writelnf!("{:032b}", a & b);
    writelnf!("{:032b}", a | b);
    writelnf!("{:032b}", a ^ b);
}
