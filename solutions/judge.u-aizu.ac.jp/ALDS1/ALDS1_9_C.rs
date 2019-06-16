#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let h = &mut DHeap::new(2, Vec::new(), PartialOrd::gt);

    loop {
        let b: StringBuffer = read!();
        let s: String = From::from(b);
        let command = s.as_ref();
        if command == b"end" {
            break;
        } else if command == b"insert" {
            let k: usize = read!();
            heap::insert(h, k);
        } else if command == b"extract" {
            writelnf!("{:d}", heap::extract(h));
        }
    }
}
