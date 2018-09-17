#![feature(use_extern_macros)]
extern crate trait_tests;
use trait_tests::*;

extern crate porus;
use porus::prelude::*;

use std::mem::size_of;

trait HandleNonNull: pool::Handle {}

#[trait_tests]
trait HandleNonNullTests: HandleNonNull {
    fn test() {
        assert!(size_of::<Self>() == size_of::<Option<Self>>());
    }
}

mod test_alloc {
    use super::*;
    use porus::alloc::Handle;

    #[test_impl]
    impl HandleNonNull for Handle {}
}

mod test_chunk {
    use super::*;
    use porus::chunk::Handle;

    #[test_impl]
    impl HandleNonNull for Handle {}
}
