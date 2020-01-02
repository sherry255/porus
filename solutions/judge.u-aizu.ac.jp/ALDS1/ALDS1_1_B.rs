#[macro_use]
extern crate porus;
prelude!();

fn main() {
    let n: usize = read!();
    let a: &mut Vec<isize> = &mut (0..n).map(|_| read!()).collect();


    let x  = a.iter().next();
    let y = a.iter().next();
//    let m = a.iter().min();
    let mut j;
    for index in 2..x {
        if (x%index==0) && (y%index==0){
            j=index;
        }
    }
    writelnf!("{:d}", j);
}