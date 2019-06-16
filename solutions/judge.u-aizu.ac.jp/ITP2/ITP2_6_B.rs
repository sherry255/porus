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
    let a: &mut Vec<isize> = &mut (0..n).map(|_| read!()).collect();
    let m: usize = read!();
    let b: &mut Vec<isize> = &mut (0..m).map(|_| read!()).collect();

    if includes(list::iter(a), list::iter(b)) {
        writelnf!("1");
    } else {
        writelnf!("0");
    }
}
