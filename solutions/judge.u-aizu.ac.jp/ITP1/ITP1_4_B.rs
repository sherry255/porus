#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let r: f64 = read!();
    writelnf!("{:.6f} {:.6f}", PI * r * r, PI * 2.0 * r);
}
