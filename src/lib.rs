#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use crate::ast::LitIntegerOrExprs;
use crate::macros::debug_eprintln;
use proc_macro::{self, TokenStream};
use tnconst_impl::{
    nconst_impl_lit_integer, nconst_impl_math_exprs, pconst_impl_lit_integer,
    pconst_impl_math_exprs, tnconst_impl_lit_integer, tnconst_impl_math_exprs,
    uconst_impl_lit_integer, uconst_impl_math_exprs,
};

mod ast;
mod ast_macro;
mod exprs_impl;
mod macros;
mod tnconst_impl;
mod uconst_impl;

#[proc_macro]
pub fn uconst(items: TokenStream) -> TokenStream {
    match syn::parse::<LitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs {
            LitIntegerOrExprs::Exprs(math_exprs) => uconst_impl_math_exprs(math_exprs)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
            LitIntegerOrExprs::LitInteger(lit_integer) => uconst_impl_lit_integer(lit_integer)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
        },
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn tnconst(items: TokenStream) -> TokenStream {
    match syn::parse::<LitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs {
            LitIntegerOrExprs::Exprs(math_exprs) => tnconst_impl_math_exprs(math_exprs)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
            LitIntegerOrExprs::LitInteger(lit_integer) => tnconst_impl_lit_integer(lit_integer)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
        },
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn pconst(items: TokenStream) -> TokenStream {
    match syn::parse::<LitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs {
            LitIntegerOrExprs::Exprs(math_exprs) => pconst_impl_math_exprs(math_exprs)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
            LitIntegerOrExprs::LitInteger(lit_integer) => pconst_impl_lit_integer(lit_integer)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
        },
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn nconst(items: TokenStream) -> TokenStream {
    match syn::parse::<LitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs {
            LitIntegerOrExprs::Exprs(math_exprs) => nconst_impl_math_exprs(math_exprs)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
            LitIntegerOrExprs::LitInteger(lit_integer) => nconst_impl_lit_integer(lit_integer)
                .unwrap_or_else(syn::Error::into_compile_error)
                .into(),
        },
        Err(err) => err.into_compile_error().into(),
    }
}
