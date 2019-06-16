#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<usize> = &mut (0..n).map(|_| read!()).collect();
    let q: usize = read!();

    for _ in 0..q {
        let k: usize = read!();
        let r = list::bsearch(a, &k);
        writelnf!("{:d}", r.0);
    }
}
