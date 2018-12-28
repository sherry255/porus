#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut s: StringBuffer = read!();
    let q: usize = read!();

    for _ in 0..q {
        let buf: StringBuffer = read!();
        let sc: String = From::from(buf);
        let command = sc.as_ref();
        let a: usize = read!();
        let b: usize = read!();

        if command == b"replace" {
            read!(s.as_mut()[a..=b].as_mut());
        } else if command == b"reverse" {
            s.as_mut()[a..=b].reverse();
        } else if command == b"print" {
            writelnf!("{:s}", unsafe {
                core::str::from_utf8_unchecked(s.as_ref()[a..=b].as_ref())
            });
        }
    }
}
