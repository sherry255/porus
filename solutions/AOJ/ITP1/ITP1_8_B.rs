#[macro_use]
extern crate porus;
prelude!();

fn main() {
    loop {
        let mut sum: usize = 0;
        let mut c: u8 = 0;
        read(Whitespace);
        while read(Char(&mut c)) && (c >= b'0') && (c <= b'9') {
            sum += (c - b'0') as usize;
        }

        if sum == 0 {
            break;
        }

        writelnf!("{:d}", sum)
    }
}
