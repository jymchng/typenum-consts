use crate::types::LitInteger;
use crate::uconst_impl;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{LitInt, Result};

pub(crate) fn tnconst_impl(lit_integer: LitInteger) -> Result<TokenStream2> {
    match lit_integer {
        LitInteger::Unsigned { lit_integer } => uconst_impl::uconst_impl(lit_integer),
        LitInteger::Positive { lit_integer } => pconst_impl(lit_integer),
        LitInteger::Negative { lit_integer } => nconst_impl(lit_integer),
    }
}

pub(crate) fn pconst_impl(lit_integer: LitInt) -> Result<TokenStream2> {
    let unsigned_ts = uconst_impl::uconst_impl(lit_integer)?;
    Ok(quote!(
        ::typenum::PInt<#unsigned_ts>
    ))
}

pub(crate) fn nconst_impl(lit_integer: LitInt) -> Result<TokenStream2> {
    let unsigned_ts = uconst_impl::uconst_impl(lit_integer)?;
    Ok(quote!(
        ::typenum::NInt<#unsigned_ts>
    ))
}
