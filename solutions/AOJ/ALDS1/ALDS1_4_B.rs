#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let s = &mut Array::<usize>::new_from_iter((0..n).map(|_| read!()));
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
