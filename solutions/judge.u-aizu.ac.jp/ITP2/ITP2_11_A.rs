#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: u64 = read!();

    for i in 0..1 << n {
        writef!("{:d}:", i);
        for j in 0..n {
            if ((1 << j) & i) > 0 {
                writef!(" {:d}", j)
            }
        }
        writelnf!("");
    }
}
