use proc_macro2::{Span, TokenStream};
use syn::parse::{ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use syn::{Expr, ExprTuple, LitStr};

fn args(input: ParseStream) -> Result<(LitStr, Expr)> {
    let s: LitStr = input.parse()?;

    if !input.is_empty() {
        let _: Comma = input.parse()?;
    }

    let args = Punctuated::parse_terminated(input)?;

    let tuple = Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: Paren(Span::call_site()),
        elems: args,
    });

    Ok((s, tuple))
}

pub fn parse_args(tokens: TokenStream) -> Result<(LitStr, Expr)> {
    Parser::parse2(args, tokens)
}
