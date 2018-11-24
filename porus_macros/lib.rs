#![feature(rustc_private)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate fmt_macros;

use proc_macro::TokenStream;

mod common;
mod format;

#[proc_macro]
pub fn format(stream: TokenStream) -> TokenStream {
    format::format(stream.into()).into()
}
