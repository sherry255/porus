#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let k: usize = read!();
    let b: Vec<usize> = (0..k).map(|_| read!()).collect();

    for i in 0..1 << k {
        writef!(
            "{:d}:",
            (0..k).fold(0, |sum, d| if ((1 << d) & i) > 0 {
                sum | (1 << b[d])
            } else {
                sum
            })
        );
        for j in 0..k {
            if ((1 << j) & i) > 0 {
                writef!(" {:d}", b[j])
            }
        }
        writelnf!("");
    }
}
