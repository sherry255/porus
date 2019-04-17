#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let a = &mut DoublyLinkedList::new_with_pool(Chunk::<_>::new_with_capacity(100000));

    for _ in 0..n {
        let b: StringBuffer = read!();
        let name: String = From::from(b);
        let time: usize = read!();
        deque::push_back(a, (name, time));
    }

    let mut sum: usize = 0;

    while !deque::is_empty(a) {
        let (name, time) = deque::pop_front(a);
        if time <= q {
            sum += time;
            writelnf!("{:s} {:d}", &name, sum);
        } else {
            sum += q;
            deque::push_back(a, (name, time - q));
        }
    }
}
