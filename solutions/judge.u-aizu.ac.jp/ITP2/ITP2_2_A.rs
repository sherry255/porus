#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a: &mut Vec<_> = &mut (0..n).map(|_| Vec::new()).collect();

    for _ in 0..q {
        let op: usize = read!();
        let t: usize = read!();
        if op == 0 {
            let x: isize = read!();
            stack::push(list::get_mut(a, t), x);
        } else if op == 1 {
            if !stack::is_empty(list::get(a, t)) {
                writelnf!("{:d}", stack::top(list::get(a, t)))
            }
        } else if op == 2 {
            Stack::pop(list::get_mut(a, t));
        }
    }
}
