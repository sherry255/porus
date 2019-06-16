#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let s: &mut Vec<usize> = &mut (0..n).map(|_| read!()).collect();
    let q: usize = read!();

    let mut count = 0;

    for _ in 0..q {
        let t: usize = read!();
        if let Some(_) = list::find(s, &t) {
            count += 1;
        }
    }

    writelnf!("{:d}", count);
}
