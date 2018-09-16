#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let n: usize = read!();

    let list = &mut DoublyLinkedList::new_with_pool(Chunk::<_>::new_with_capacity(500000));
    let mut cursor = list.front();

    for _ in 0..n {
        let op: isize = read!();
        if op == 0 {
            let x: isize = read!();
            cursor = Some(list.insert_before(x, cursor));
        } else if op == 1 {
            let d: isize = read!();

            if d < 0 {
                for _ in 0..(-d) {
                    cursor = list.prev(cursor);
                }
            } else {
                for _ in 0..d {
                    cursor = list.next(cursor);
                }
            }
        } else if op == 2 {
            let next = list.next(cursor);
            list.remove(cursor.unwrap());
            cursor = next;
        }
    }

    while !deque::is_empty(list) {
        writelnf!("{:d}", deque::pop_front(list));
    }
}
