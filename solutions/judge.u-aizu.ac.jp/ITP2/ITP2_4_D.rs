#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &Vec<isize> = &(0..n).map(|_| read!()).collect();
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
