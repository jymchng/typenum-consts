#![no_std]

use crate::macros::debug_eprintln;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, LitInt};

mod macros;
mod no_std_formatter;
mod uconst_impl;

#[cfg(debug_assertions)]
#[cfg(feature = "debug")]
extern crate std;

#[proc_macro]
pub fn uconst(items: TokenStream) -> TokenStream {
    let litint = parse_macro_input!(items as LitInt);
    debug_eprintln!("`litint`: {}", litint);
    uconst_impl::uconst_impl(litint)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
