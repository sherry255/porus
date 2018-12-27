#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<usize>::new_from_iter((0..n).map(|_| read!()));
    let q: usize = read!();

    for _ in 0..q {
        let k: usize = read!();
        let r = list::bsearch(a, &k);
        writelnf!("{:d} {:d}", r.0, r.1);
    }
}
