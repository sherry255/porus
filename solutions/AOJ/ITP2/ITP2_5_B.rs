#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a = &mut Array::<(usize, usize, u8, usize, String)>::new_from_iter((0..n).map(|_| {
        let v = read!();
        let w = read!();
        let mut t: u8 = 0;
        read!(Char(&mut t));
        let d = read!();
        let b: StringBuffer = read!();
        let s: String = From::from(b);
        (v, w, t, d, s)
    }));

    list::shell_sort(
        a,
        &PartialOrd::lt,
        static_array![797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1],
    );

    for (v, w, t, d, s) in list::iter(a) {
        writelnf!("{:d} {:d} {:c} {:d} {:s}", v, w, t, d, &*s)
    }
}
