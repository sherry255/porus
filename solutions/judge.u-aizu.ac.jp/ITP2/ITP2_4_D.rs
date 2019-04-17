#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &Array::<isize>::new_from_iter((0..n).map(|_| read!()));
    let mut old = None;

    writelnf!(
        "{}",
        join(
            f!(" "),
            list::iter(a).filter_map(|x| if old == Some(x) {
                None
            } else {
                old = Some(x);
                Some(f!("{:d}", x))
            })
        )
    );
}
