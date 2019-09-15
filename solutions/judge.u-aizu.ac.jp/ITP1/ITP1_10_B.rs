#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let a: f64 = read!();
    let b: f64 = read!();
    let c: f64 = read!();

    let d = f64::to_radians(c);
    let h = b * sin(d);
    let w = b * cos(d);

    writelnf!("{:.5f}", a * h / 2.0);
    writelnf!("{:.5f}", (a + b + sqrt(h * h + (a - w) * (a - w))));
    writelnf!("{:.5f}", h);
}
