#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(const_fn)]
#![feature(const_slice_len)]
#![feature(try_from)]
#![feature(specialization)]
#![feature(refcell_replace_swap)]
#![feature(untagged_unions)]
#![feature(custom_attribute)]
#![cfg_attr(feature = "online-judge", feature(lang_items))]
#![doc(test(attr(feature(proc_macro_hygiene))))]
#![no_std]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::else_if_without_else)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::multiple_inherent_impl)]
#![deny(clippy::print_stdout)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_same)]
#![deny(clippy::string_add)]
#![deny(clippy::unimplemented)]
#![deny(clippy::use_debug)]
#![deny(clippy::wrong_pub_self_convention)]

//! [`porus`](self) is a library for competitive programming. Since
//! most popular online judges accept only a single file within tens
//! of kilobytes, solutions have to be preproccessed before submitting
//! to online judges. Right now, [`porus`](self) piggybacks on
//! [ix](https://github.com/bhuztez/ix) to do the preprocessing. For
//! example, to submit to
//! [AOJ](http://judge.u-aizu.ac.jp/onlinejudge/) the solution to
//! [ITP1_1_A](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A)
//!

//! ```bash
//! $ python3 -mix submit -w solutions/AOJ/ITP1/ITP1_1_A.rs
//! [SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs
//! [COMPILE] target/x86_64-unknown-linux-gnu/release/libporus.rlib
//! AOJ (judge.u-aizu.ac.jp)
//! User ID: your_username
//! Password:
//! [SUBMIT] solutions/AOJ/ITP1/ITP1_1_A.rs: Accepted (Memory: 2000, Time: 0, Length: 1380)
//! $
//! ```
//!

//! ## Abstract Data Types
//! * [`Pool`](pool)
//! * [`Allocator`](allocator)
//! * [`Collection`](collection)
//! * [`List`](list)
//! * [`Stack`](stack)
//! * [`Deque`](deque)
//!

//! ## Data Structures
//! * [`Array`](type@array) : [`List`](list) + [`Stack`](stack)
//! * [`Buffer`](type@buffer) : [`List`](list) + [`Deque`](deque)
//! * [`DoublyLinkedList`](type@dlist) : [`Deque`](deque)
//!

extern crate porus_macros;

pub mod file;
pub mod libc;
pub mod os;
pub mod stdio;

pub mod fmt;
pub mod io;
pub mod scan;

pub mod allocator;
pub mod capacity;
pub mod collection;
pub mod deque;
pub mod list;
pub mod pool;
pub mod stack;
pub mod stream;

pub mod block;
pub mod chunk;
pub mod string;
#[macro_use]
pub mod static_array;
#[macro_use]
pub mod array;
#[macro_use]
pub mod buffer;
pub mod dlist;
pub mod flist;

#[macro_use]
pub mod prelude;

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
