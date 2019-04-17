#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a = &mut Array::<_>::new_from_iter((0..n).map(|_| Array::<isize>::new()));

    for _ in 0..q {
        let op: usize = read!();
        let t: usize = read!();
        if op == 0 {
            let x: isize = read!();
            stack::push(list::get_mut(a, t), x);
        } else if op == 1 {
            writelnf!(
                "{}",
                join(f!(" "), list::iter(list::get(a, t)).map(|e| f!("{:d}", e)))
            )
        } else if op == 2 {
            list::set(a, t, Array::<isize>::new());
        }
    }
}
