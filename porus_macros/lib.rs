#![feature(rustc_private)]

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
