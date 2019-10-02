#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(const_fn)]
#![feature(specialization)]
#![feature(untagged_unions)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(maybe_uninit_ref)]
#![feature(ptr_offset_from)]
#![feature(is_sorted)]
#![feature(const_saturating_int_methods)]
#![cfg_attr(feature = "online-judge", feature(lang_items))]
#![doc(test(attr(feature(proc_macro_hygiene))))]
#![no_std]
#![deny(stable_features)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]
#![deny(clippy::restriction)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::use_self)]

//! [`porus`](self) is a library for competitive programming. Since
//! most popular online judges accept only a single file within tens
//! of kilobytes, solutions have to be preproccessed before submitting
//! to online judges. Right now, [`porus`](self) piggybacks on
//! [wronganswer](https://github.com/bhuztez/wronganswer) to do the
//! preprocessing. For example, to submit to
//! [AOJ](http://judge.u-aizu.ac.jp/onlinejudge/) the solution to
//! [ITP1_1_A](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A)
//!

//! ```bash
//! $ ./c.py submit solutions/judge.u-aizu.ac.jp/ITP1/ITP1_1_A.rs
//! Memory: 2068, Time: 0, Length: 4344
//! $
//! ```
//!

//! ## Abstract Data Types
//! * [`Pool`](pool)
//! * [`Collection`](collection)
//! * [`List`](list)
//! * [`Stack`](stack)
//! * [`Deque`](deque)
//!

//! ## Data Structures
//!

extern crate alloc;
extern crate porus_macros;

pub mod utils;

pub mod math;

pub mod file;
pub mod libc;
pub mod stdio;

pub mod fmt;
pub mod io;
pub mod scan;

pub mod allocator;
pub mod capacity;
pub mod collection;
pub mod deque;
pub mod heap;
pub mod list;
pub mod pool;
pub mod stack;

pub mod block;
pub mod chunk;
pub mod string;

pub mod dheap;
pub mod dlist;
pub mod flist;

#[macro_use]
pub mod prelude;

#[allow(clippy::missing_const_for_fn)]
#[cfg(feature = "online-judge")]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(feature = "online-judge")]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe { ::core::intrinsics::abort() }
}

#[global_allocator]
static _A: allocator::System = allocator::System;

#[cfg(feature = "online-judge")]
#[alloc_error_handler]
fn oom(_info: ::core::alloc::Layout) -> ! {
    unsafe { ::core::intrinsics::abort() }
}
