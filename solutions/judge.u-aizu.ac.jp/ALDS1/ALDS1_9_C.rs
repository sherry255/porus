#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let h = &mut BinaryHeap::new();

    loop {
        let b: StringBuffer = read!();
        let s: String = From::from(b);
        let command = s.as_ref();
        if command == b"end" {
            break;
        } else if command == b"insert" {
            let k: usize = read!();
            heap::push(h, k);
        } else if command == b"extract" {
            writelnf!("{:d}", heap::pop(h));
        }
    }
}
