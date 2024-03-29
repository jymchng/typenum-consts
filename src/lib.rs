#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use crate::ast::{
    LitIntegerOrExprs, NegativeLitIntegerOrExprs, PositiveLitIntegerOrExprs,
    UnsignedLitIntegerOrExprs,
};
use crate::macros::debug_eprintln;
use proc_macro::{self, TokenStream};
use tnconst_impl::{
    nconst_impl_lit_integer, nconst_impl_math_exprs, pconst_impl_lit_integer,
    pconst_impl_math_exprs, tnconst_impl_lit_integer, tnconst_impl_math_exprs,
    uconst_impl_lit_integer, uconst_impl_math_exprs,
};
#[allow(clippy::all)]
use vendors::rsc;

mod ast;
mod ast_macro;
mod exprs_impl;
mod macros;
mod tnconst_impl;
mod uconst_impl;
mod vendors;

/// [`uconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`](https://github.com/paholg/typenum/tree/main/src)'s type-level unsigned integer (i.e. the type implements the `typenum::Unsigned` trait).
/// There are three ways you can invoke this macro.
///
/// ## 1. Invoke it with a literal integer
///
/// ```rust
/// use typenum::{U123, assert_type_eq};
/// use typenum_consts::uconst;
///
/// type A = uconst![123];
/// assert_type_eq!(A, U123);
/// ```
///
/// Compilation fails if the literal integer is prefixed with either a `-` or a `+`.
///
/// ```compile_fail
/// # use typenum::{U123, assert_type_eq};
/// # use typenum_consts::uconst;
/// type B = uconst![+123]; // Fail to compile
/// ```
///
/// ```compile_fail
/// # use typenum::{U123, assert_type_eq};
/// # use typenum_consts::uconst;
/// type C = uconst![-123]; // Fail to compile
/// ```
///
/// ## 2. Invoke using an expression or many simple mathematical expressions
/// ```rust
/// use typenum::{U15, assert_type_eq};
/// use typenum_consts::uconst;
/// type D = uconst![{
///     a = 10;
///     b = 5;
///     a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, U15
/// }];
/// assert_type_eq!(D, U15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `uconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`.
/// ```rust
/// use typenum::{U69, assert_type_eq};
/// use typenum_consts::uconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type E = uconst![env!("ENV_VAR");];
/// assert_type_eq!(E, U69);
/// ```
#[proc_macro]
pub fn uconst(items: TokenStream) -> TokenStream {
    match syn::parse::<UnsignedLitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs.0 {
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

/// [`pconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`](https://github.com/paholg/typenum/tree/main/src)'s type-level positive integer.
/// There are three ways you can invoke this macro.
///
/// ## 1. Invoke it with a literal integer
///
/// ```rust
/// use typenum::{P123, assert_type_eq};
/// use typenum_consts::pconst;
///
/// type A = pconst![123];
/// assert_type_eq!(A, P123);
/// ```
///
/// Compilation fails if the literal integer is prefixed with a `-` or a `+`.
///
/// ```compile_fail
/// # use typenum::{P123, assert_type_eq};
/// # use typenum_consts::pconst;
/// type B = pconst![-123]; // Fail to compile
/// type C = pconst![+123]; // Fail to compile
/// ```
///
/// It is a **must** to not prefix the literal integer with a `+`.
///
/// ```rust
/// use typenum::{P123, assert_type_eq};
/// use typenum_consts::pconst;
///
/// type A = pconst![123];
/// assert_type_eq!(A, P123);
/// ```
///
/// ## 2. Invoke using an expression or many simple mathematical expressions
/// ```rust
/// use typenum::{P15, assert_type_eq};
/// use typenum_consts::pconst;
/// type D = pconst![{
///     a = 10;
///     b = 5;
///     a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, P15
///     // In this case, it must always be positive or else compilation will fail.
/// }];
/// assert_type_eq!(D, P15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `pconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`.
/// ```rust
/// use typenum::{P69, assert_type_eq};
/// use typenum_consts::pconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type E = pconst![env!("ENV_VAR");];
/// assert_type_eq!(E, P69);
/// ```
#[proc_macro]
pub fn pconst(items: TokenStream) -> TokenStream {
    match syn::parse::<PositiveLitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs.0 {
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
    match syn::parse::<NegativeLitIntegerOrExprs>(items) {
        Ok(litint_exprs) => match litint_exprs.0 {
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
