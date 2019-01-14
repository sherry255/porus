#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a = &mut Array::<_>::new_from_iter(
        (0..n).map(|_| DHeap::new(2, Array::<_>::new(), PartialOrd::gt)),
    );

    for _ in 0..q {
        let op: usize = read!();
        let t: usize = read!();
        let h = list::get_mut(a, t);
        if op == 0 {
            let x: isize = read!();
            heap::insert(h, x);
        } else if op == 1 {
            if let Some(&x) = Heap::top(h) {
                writelnf!("{:d}", x)
            }
        } else if op == 2 {
            Heap::extract(h);
        }
    }
}
