#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: isize = read!();
    let b: isize = read!();
    writelnf!(
        "a {:s} b",
        match Ord::cmp(&a, &b) {
            Less => "<",
            Equal => "==",
            Greater => ">",
        }
    );
}
