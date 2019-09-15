pub use alloc::vec;
pub use alloc::vec::Vec;
pub use alloc::collections::VecDeque;
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
pub use crate::scan::{Char, Whitespace};
pub use crate::stdio::{read, read_opt, writef, writelnf};

pub use crate::allocator;
pub use crate::pool;

pub use crate::allocator::Pool;
pub use crate::chunk::Chunk;

pub use crate::collection::{self, Collection};
pub use crate::deque::{self, Deque};
pub use crate::heap::{self, Heap};
pub use crate::list::sorting;
pub use crate::list::{self, List};
pub use crate::stack::{self, Stack};

pub use crate::array::{array, Array};
pub use crate::buffer::{buffer, Buffer};
pub use crate::dheap::{self, DHeap};
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

        pub mod __porus_main {
            use $crate::file::{Sink, Source};
            use $crate::stdio::initialize;

            static mut STDIN: [u8; $size] = [0; $size];
            static mut STDOUT: [u8; $size] = [0; $size];

            #[cfg_attr(not(feature = "online-judge"), main)]
            fn main() {
                let stdin = &mut Source::new(0, unsafe { &mut STDIN });
                let stdout = &mut Sink::new(1, unsafe { &mut STDOUT });
                initialize(stdin, stdout);
                ::main();
            }

            #[cfg(feature = "online-judge")]
            #[export_name = "main"]
            pub extern "C" fn porus_start() -> i32 {
                main();
                0
            }
        }
    };
}
