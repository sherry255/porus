#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let h: usize = read!();
    let v = &mut Array::<isize>::new_from_iter((0..h).map(|_| read!()));

    let write_key = |name: &str, i: usize| {
        writef!("{:s} key = {:d}, ", name, list::get(v, i));
    };

    for i in 0..h {
        writef!("node {:d}:", i + 1);
        write_key("", i);
        if i > 0 {
            write_key("parent", dheap::parent_index(2, i));
        }
        let left = dheap::child_index(2, i, 0);
        let right = dheap::child_index(2, i, 1);
        if left < h {
            write_key("left", left);
        }
        if right < h {
            write_key("right", right);
        }
        writelnf!("");
    }
}
