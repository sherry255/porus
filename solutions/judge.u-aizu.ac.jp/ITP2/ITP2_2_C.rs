#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a: &mut Vec<_> = &mut (0..n).map(|_| BinaryHeap::new()).collect();

    for _ in 0..q {
        let op: usize = read!();
        let t: usize = read!();
        let h = list::get_mut(a, t);
        if op == 0 {
            let x: isize = read!();
            heap::push(h, x);
        } else if op == 1 {
            if let Some(&x) = Heap::peek(h) {
                writelnf!("{:d}", x)
            }
        } else if op == 2 {
            Heap::pop(h);
        }
    }
}
