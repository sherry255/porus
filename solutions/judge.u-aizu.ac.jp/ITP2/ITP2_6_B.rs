#[macro_use]
extern crate porus;
prelude!();

fn includes(mut a: impl Iterator<Item = isize>, b: impl Iterator<Item = isize>) -> bool {
    let mut x = 0;
    for y in b {
        loop {
            if let Some(z) = a.next() {
                x = z;
            } else {
                return false;
            }

            if y < x {
                return false;
            }

            if y == x {
                break;
            }
        }
    }

    return true;
}

fn main() {
    let n: usize = read!();
    let a = &mut Array::<isize>::new_from_iter((0..n).map(|_| read!()));
    let m: usize = read!();
    let b = &mut Array::<isize>::new_from_iter((0..m).map(|_| read!()));

    if includes(list::iter(a), list::iter(b)) {
        writelnf!("1");
    } else {
        writelnf!("0");
    }
}
