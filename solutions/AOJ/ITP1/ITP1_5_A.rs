#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    loop {
        let h: usize = read!();
        let w: usize = read!();
        if (h == 0) && (w == 0) {
            break;
        }

        for _ in 0..h {
            for _ in 0..w {
                writef!("#");
            }
            writelnf!("");
        }
        writelnf!("");
    }
}
