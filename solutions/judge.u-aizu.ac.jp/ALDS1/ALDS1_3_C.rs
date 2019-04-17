#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();

    let a = &mut DoublyLinkedList::new_with_pool(Chunk::<_>::new_with_capacity(1000000));

    for _ in 0..n {
        let b: StringBuffer = read!();
        let s: String = From::from(b);
        let command = s.as_ref();
        if command == b"insert" {
            let x: usize = read!();
            a.insert_after(x, None);
        } else if command == b"delete" {
            let x: usize = read!();
            let mut h = a.front();
            while !h.is_none() {
                if a.get(h.unwrap()) == &x {
                    a.remove(h.unwrap());
                    break;
                }
                h = a.next(h);
            }
        } else if command == b"deleteFirst" {
            let h = a.front().unwrap();
            a.remove(h);
        } else if command == b"deleteLast" {
            let h = a.back().unwrap();
            a.remove(h);
        }
    }

    writelnf!("{}", join(f!(" "), deque::drain(a).map(|e| f!("{:d}", e))));
}
