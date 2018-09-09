#![feature(rustc_private)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate fmt_macros;

use proc_macro::{Group, Span, TokenStream, TokenTree};
use std::iter::FromIterator;

mod common;
mod format;

fn set_span(span: Span, stream: TokenStream) -> TokenStream {
    let iter = stream.into_iter().map(|mut tree| match tree {
        TokenTree::Group(g) => {
            TokenTree::Group(Group::new(g.delimiter(), set_span(span, g.stream())))
        }
        _ => {
            tree.set_span(span);
            tree
        }
    });
    TokenStream::from_iter(iter)
}

#[proc_macro]
pub fn f(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::f(stream.into()).into())
}

#[proc_macro]
pub fn writef(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::writef(stream.into()).into())
}

#[proc_macro]
pub fn writelnf(stream: TokenStream) -> TokenStream {
    set_span(Span::call_site(), format::writelnf(stream.into()).into())
}
