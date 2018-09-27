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

pub use crate::stdio;

pub use crate::fmt::{f, join};
pub use crate::io;
pub use crate::scan::Char;
pub use crate::stdio::{read, read_opt, writef, writelnf};

pub use crate::allocator;
pub use crate::pool;

pub use crate::allocator::Pool;
pub use crate::chunk::Chunk;

pub use crate::collection;
pub use crate::deque;
pub use crate::list;
pub use crate::stack;

pub use crate::array::{array, Array};
pub use crate::buffer::{buffer, Buffer};
pub use crate::dlist::DoublyLinkedList;
pub use crate::flist::SinglyLinkedList;
pub use crate::static_array::static_array;
pub use crate::string::{stringf, String, StringBuffer};

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        prelude!(1024);
    };
    ($size:expr) => {
        #[allow(unused_imports)]
        use $crate::prelude::*;

        mod porus_main {
            use $crate::file::{Sink, Source};
            use $crate::stdio::initialize;

            static mut STDIN: [u8; $size] = [0; $size];
            static mut STDOUT: [u8; $size] = [0; $size];

            pub fn main() {
                let stdin = &mut Source::new(0, unsafe { &mut STDIN });
                let stdout = &mut Sink::new(1, unsafe { &mut STDOUT });
                initialize(stdin, stdout);
                ::solve();
            }
        }

        #[cfg(not(feature = "online-judge"))]
        fn main() {
            porus_main::main();
        }

        #[cfg(feature = "online-judge")]
        #[no_mangle]
        pub extern "C" fn main() -> i32 {
            porus_main::main();
            0
        }
    };
}
