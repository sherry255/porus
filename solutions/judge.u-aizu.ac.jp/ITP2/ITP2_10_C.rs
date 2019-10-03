#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let mut state: u64 = 0;
    let q: usize = read!();
    for _ in 0..q {
        let query: usize = read!();

        match query {
            0 => {
                let i: usize = read!();
                writelnf!("{:d}", if (state & (1 << i)) > 0 { 1 } else { 0 })
            }
            1 => {
                let i: usize = read!();
                state |= 1 << i;
            }
            2 => {
                let i: usize = read!();
                state &= !(1 << i);
            }
            3 => {
                let i: usize = read!();
                state ^= 1 << i;
            }
            4 => {
                writelnf!("{:d}", if state == u64::max_value() { 1 } else { 0 });
            }
            5 => {
                writelnf!("{:d}", if state > 0 { 1 } else { 0 });
            }
            6 => {
                writelnf!("{:d}", if state == 0 { 1 } else { 0 });
            }
            7 => {
                writelnf!("{:d}", state.count_ones());
            }
            8 => {
                writelnf!("{:d}", state);
            }
            _ => panic!("invalid query"),
        }
    }
}
