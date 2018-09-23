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

        for i in 0..h {
            for j in 0..w {
                writef!(
                    "{:s}",
                    match (i % 2) == (j % 2) {
                        false => ".",
                        true => "#",
                    }
                );
            }
            writelnf!("");
        }
        writelnf!("");
    }
}
