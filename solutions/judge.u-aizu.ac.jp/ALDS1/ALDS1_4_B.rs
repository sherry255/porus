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
        let r = list::bsearch(s, &t);
        if r.1 > r.0 {
            count += 1;
        }
    }

    writelnf!("{:d}", count);
}
