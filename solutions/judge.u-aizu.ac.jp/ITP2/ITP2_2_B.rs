#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a = &mut Array::<_>::new_from_iter((0..n).map(|_| Buffer::<isize>::new()));

    for _ in 0..q {
        let op: usize = read!();
        let t: usize = read!();
        if op == 0 {
            let x: isize = read!();
            deque::push_back(list::get_mut(a, t), x);
        } else if op == 1 {
            if !deque::is_empty(list::get(a, t)) {
                writelnf!("{:d}", deque::front(list::get(a, t)))
            }
        } else if op == 2 {
            Deque::pop_front(list::get_mut(a, t));
        }
    }
}
