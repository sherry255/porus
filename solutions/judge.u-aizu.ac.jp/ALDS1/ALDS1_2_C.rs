#[macro_use]
extern crate porus;
prelude!();

#[derive(Default, Clone, Copy)]
struct Card(u8, u8);

fn read_card() -> Card {
    let mut suit = 0u8;
    read!(Char(&mut suit));
    Card(suit, read!())
}

fn stable(b: bool) -> &'static str {
    if b {
        "Stable"
    } else {
        "Not stable"
    }
}

fn main() {
    let n: usize = read!();
    let a: &mut Vec<Card> = &mut (0..n).map(|_| read_card()).collect();

    let bi: &mut Vec<usize> = &mut (0..n).collect();
    list::bubble_sort(bi, |&i, &j| list::get(a, i).1 < list::get(a, j).1);
    writelnf!(
        "{}",
        join(
            f!(" "),
            list::iter(bi).map(|i| f!("{:c}{:d}", list::get(a, i).0, list::get(a, i).1))
        )
    );
    writelnf!(
        "{:s}",
        stable(list::is_stable_sort(a, |x, y| x.1 < y.1, bi))
    );

    let si: &mut Vec<usize> = &mut (0..n).collect();
    list::selection_sort(si, |&i, &j| list::get(a, i).1 < list::get(a, j).1);
    writelnf!(
        "{}",
        join(
            f!(" "),
            list::iter(si).map(|i| f!("{:c}{:d}", list::get(a, i).0, list::get(a, i).1))
        )
    );
    writelnf!(
        "{:s}",
        stable(list::is_stable_sort(a, |x, y| x.1 < y.1, si))
    );
}
