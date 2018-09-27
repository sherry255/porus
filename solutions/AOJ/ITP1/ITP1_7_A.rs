#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let m: isize = read!();
        let f: isize = read!();
        let r: isize = read!();

        if (m == -1) && (f == -1) && (r == -1) {
            break;
        }

        if (m == -1) || (f == -1) {
            writelnf!("F");
        } else {
            writelnf!(
                "{:s}",
                match m + f {
                    80...100 => "A",
                    65..80 => "B",
                    50..65 => "C",
                    30..50 if r >= 50 => "C",
                    30..50 => "D",
                    0..30 => "F",
                    _ => panic!(),
                }
            );
        }
    }
}
