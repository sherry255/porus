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
        if let Some(_) = list::find(s, &t) {
            count += 1;
        }
    }

    writelnf!("{:d}", count);
}
