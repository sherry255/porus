#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut c: u8 = 0;
    while read(Char(&mut c)) {
        writef!(
            "{:c}",
            match c {
                b'a'...b'z' => c - b'a' + b'A',
                b'A'...b'Z' => c - b'A' + b'a',
                _ => c,
            }
        )
    }
}
