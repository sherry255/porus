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

pub use porus::io;
pub use porus::io::read::Char;
pub use porus::io::write::join;
pub use porus_macros::f;

pub use porus::alloc;
pub use porus::pool;

pub use porus::alloc::Pool;
pub use porus::chunk::Chunk;

pub use porus::collection;
pub use porus::deque;
pub use porus::list;
pub use porus::stack;

pub use porus::array::Array;
pub use porus::dlist::DoublyLinkedList;
pub use porus::flist::SinglyLinkedList;
pub use porus::string::{String, StringBuffer};

/// the porus prelude
#[macro_export]
macro_rules! prelude {
    () => {
        prelude!(1024);
    };
    ($size:expr) => {
        #[allow(unused_imports)]
        use porus::prelude::*;

        mod main {
            use porus::io::initialize;
            use porus::os::file::{FileSink, FileSource};

            static mut STDIN: [u8; $size] = [0; $size];
            static mut STDOUT: [u8; $size] = [0; $size];

            pub fn main() {
                let stdin = &mut FileSource::new(0, unsafe { &mut STDIN });
                let stdout = &mut FileSink::new(1, unsafe { &mut STDOUT });
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
