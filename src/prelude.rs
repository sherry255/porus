pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;
pub use core::cmp::Ordering::Less;

pub use core::f64::consts::PI;
use core::intrinsics::sqrtf64;

pub fn sqrt(x: f64) -> f64 {
    unsafe { sqrtf64(x) }
}

pub use porus::collection;
pub use porus::deque;
pub use porus::list;
pub use porus::stack;

pub use porus::array::Array;
pub use porus::dlist::DoublyLinkedList;
pub use porus::flist::SinglyLinkedList;
pub use porus::string::{String, StringBuffer};

pub use porus::io::read::Char;
pub use porus::io::write::join;
pub use porus_macros::{f, writef, writelnf};

pub fn default<T: Default>() -> T {
    Default::default()
}

#[macro_export]
macro_rules! read_opt {
    () => {{
        let mut x = Default::default();
        if ::io::read_skip_ws(&mut x) {
            Some(x)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! read {
    () => (
        {
            read_opt!().unwrap()
        }
    );
    ( $($expr:expr),* ) => (
        $(
            ::io::read_skip_ws($expr);
        )*
    )
}

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        prelude!(1024);
    };
    ($size:expr) => {
        #[allow(unused_imports)]
        use porus::prelude::*;

        mod io {
            #[cfg(debug_assertions)]
            use std::ptr::drop_in_place;

            #[cfg(not(debug_assertions))]
            use core::ptr::drop_in_place;

            use porus::io::read::{fread, Consumer, Whitespace};
            use porus::io::stdio;
            use porus::io::write::fwrite;
            use porus::io::Sink;

            #[allow(dead_code)]
            static mut STDIN: stdio::Input = stdio::stdin(&mut [0; $size]);
            static mut STDOUT: stdio::Output = stdio::stdout(&mut [0; $size]);

            #[allow(dead_code)]
            pub fn read<C: Consumer>(c: C) -> bool {
                unsafe { fread(&mut STDIN, c) }
            }

            pub fn read_skip_ws<C: Consumer>(c: C) -> bool {
                read(Whitespace);
                read(c)
            }

            #[allow(dead_code)]
            pub fn write<F: FnMut(&mut stdio::Output)>(f: &mut F) {
                unsafe {
                    fwrite(&mut STDOUT, f);
                }
            }

            #[allow(dead_code)]
            pub fn writeln<F: FnMut(&mut stdio::Output)>(f: &mut F) {
                write(f);
                unsafe {
                    Sink::write(&mut STDOUT, b'\n');
                }
            }

            pub fn main() {
                ::solve();
                unsafe { drop_in_place(&mut STDOUT as *mut _) };
            }
        }

        #[cfg(debug_assertions)]
        fn main() {
            io::main();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern "C" fn main() -> i32 {
            io::main();
            0
        }
    };
}
