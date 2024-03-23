#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use crate::macros::{debug_eprintln, no_std_format};
use crate::types::LitInteger;
use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, Error, LitInt};

mod macros;
mod no_std_formatter;
mod tnconst_impl;
mod types;
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

#[proc_macro]
pub fn tnconst(items: TokenStream) -> TokenStream {
    let lit_integer: LitInteger = parse_macro_input!(items as LitInteger);
    tnconst_impl::tnconst_impl(lit_integer)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn pconst(items: TokenStream) -> TokenStream {
    let lit_integer: LitInteger = parse_macro_input!(items as LitInteger);
    let result = match lit_integer {
        LitInteger::UnsignedInteger { lit_integer } => {
            let mut error_buf = [0u8; 512];
            Err(
                Error::new(
                    lit_integer.span(),
                    no_std_format!(&mut error_buf,
                    "the literal passed into `tnconst!` or `pconst!` needs to have a `+` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                    ).unwrap_or(
                        "unable to allocate enough memory for error message in `pconst`"
                    )
                ))
        }
        LitInteger::PositiveInteger { lit_integer } => tnconst_impl::pconst_impl(lit_integer),
        LitInteger::NegativeInteger { lit_integer } => {
            let mut error_buf = [0u8; 512];
            Err(
                Error::new(
                    lit_integer.span(),
                    no_std_format!(&mut error_buf,
                    "the literal passed into `tnconst!` or `pconst!` needs to have a `+` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                    ).unwrap_or(
                        "unable to allocate enough memory for error message in `pconst`"
                    )
                ))
        }
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

#[proc_macro]
pub fn nconst(items: TokenStream) -> TokenStream {
    let lit_integer: LitInteger = parse_macro_input!(items as LitInteger);
    let result = match lit_integer {
        LitInteger::UnsignedInteger { lit_integer } => {
            let mut error_buf = [0u8; 512];
            Err(
                Error::new(
                    lit_integer.span(),
                    no_std_format!(&mut error_buf,
                    "the literal passed into `tnconst!` or `nconst!` needs to have a `-` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                    ).unwrap_or(
                        "unable to allocate enough memory for error message in `nconst`"
                    )
                ))
        },
        LitInteger::PositiveInteger { lit_integer } => {
            let mut error_buf = [0u8; 512];
            Err(
                Error::new(
                    lit_integer.span(),
                    no_std_format!(&mut error_buf,
                    "the literal passed into `tnconst!` or `nconst!` needs to have a `-` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                    ).unwrap_or(
                        "unable to allocate enough memory for error message in `nconst`"
                    )
                ))
        },
        LitInteger::NegativeInteger { lit_integer } => tnconst_impl::nconst_impl(lit_integer),
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}