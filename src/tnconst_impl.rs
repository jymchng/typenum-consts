use crate::types::{LitInteger, MathExprs};
use crate::{exprs_impl::eval_exprs, uconst_impl};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{LitInt, Result};

pub(crate) fn tnconst_impl_lit_integer(lit_integer: LitInteger) -> Result<TokenStream2> {
    match lit_integer {
        LitInteger::Unsigned { lit_integer } => uconst_impl::uconst_impl(lit_integer),
        LitInteger::Positive { lit_integer } => pconst_impl(lit_integer),
        LitInteger::Negative { lit_integer } => nconst_impl(lit_integer),
    }
}

pub(crate) fn tnconst_impl_math_exprs(math_exprs: MathExprs) -> Result<TokenStream2> {
    tnconst_impl_lit_integer(eval_exprs(math_exprs)?)
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
