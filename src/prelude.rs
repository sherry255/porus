pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;
pub use core::cmp::Ordering::Less;

pub use core::f64::consts::PI;
use core::intrinsics::sqrtf64;

pub fn sqrt(x: f64) -> f64 {
    unsafe { sqrtf64(x) }
}

pub fn default<T: Default>() -> T {
    Default::default()
}

pub use crate::io;
pub use crate::io::read::Char;
pub use crate::io::write::join;
pub use crate::io::{f, writef, writelnf};

pub use crate::allocator;
pub use crate::pool;

pub use crate::allocator::Pool;
pub use crate::chunk::Chunk;

pub use crate::collection;
pub use crate::deque;
pub use crate::list;
pub use crate::stack;

pub use crate::array::Array;
pub use crate::dlist::DoublyLinkedList;
pub use crate::flist::SinglyLinkedList;
pub use crate::string::{String, StringBuffer};

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        prelude!(1024);
    };
    ($size:expr) => {
        #[allow(unused_imports)]
        use $crate::prelude::*;

        mod main {
            use $crate::file::{Sink, Source};
            use $crate::io::initialize;

            static mut STDIN: [u8; $size] = [0; $size];
            static mut STDOUT: [u8; $size] = [0; $size];

            pub fn main() {
                let stdin = &mut Source::new(0, unsafe { &mut STDIN });
                let stdout = &mut Sink::new(1, unsafe { &mut STDOUT });
                initialize(stdin, stdout);
                ::solve();
            }
        }

        #[cfg(debug_assertions)]
        fn main() {
            main::main();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern "C" fn main() -> i32 {
            main::main();
            0
        }
    };
}
