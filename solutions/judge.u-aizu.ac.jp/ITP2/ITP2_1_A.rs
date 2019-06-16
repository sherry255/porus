#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let vec = &mut vec![0isize; 0];

    for _ in 0..n {
        let op: usize = read!();
        if op == 0 {
            let x: isize = read!();
            stack::push(vec, x);
        } else if op == 1 {
            let p: usize = read!();
            writelnf!("{:d}", list::get(vec, p));
        } else if op == 2 {
            stack::pop(vec);
        }
    }
}
