#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let h: usize = read!();
    let v = &mut Array::<isize>::new_from_iter((0..h).map(|_| read!()));
    dheap::heapify(2, v, PartialOrd::gt);
    writelnf!("{}", join(f!(""), list::iter(v).map(|e| f!(" {:d}", e))));
}
