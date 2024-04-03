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

mod ast;
mod ast_macro;
mod exprs_impl;
mod macros;
mod tnconst_impl;
mod uconst_impl;

/// [`uconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`]'s type-level unsigned integer (i.e. the type that implements the [`typenum::Unsigned`] trait).
///
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
///
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
/// It is a compilation error if the mathematical expressions evaluate to a negative literal integer.
///
/// ```compile_fail
/// use typenum::{U15, assert_type_eq};
/// use typenum_consts::uconst;
/// type D = uconst![{
///     a = 10;
///     b = 5;
///     b - a; // 5 - 10 = -5, cannot be made into a `U15`
/// }];
/// assert_type_eq!(D, U15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
///
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `uconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`, `"ENV_VAR"` is the key to read the value from and `"./.env.prod"` is the file path relative to [`CARGO_MANIFEST_DIR`].
///
/// ```rust
/// use typenum::{U69, assert_type_eq};
/// use typenum_consts::uconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type E = uconst![env!("ENV_VAR");];
/// assert_type_eq!(E, U69);
/// ```
///
/// It is a compilation error if the environment variable evaluate to a negative literal integer.
///
/// ```compile_fail
/// use typenum::{U69, assert_type_eq};
/// use typenum_consts::uconst;
/// // ``` .env
/// // NENV_VAR=-69
/// // ```
/// type F = uconst![env!("NENV_VAR");];
/// assert_type_eq!(F, U69);
/// ```
///
/// [`CARGO_MANIFEST_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
/// [`typenum::Unsigned`]: https://docs.rs/typenum/latest/typenum/marker_traits/trait.Unsigned.html
/// [`typenum`]: https://github.com/paholg/typenum/tree/main/src
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

/// [`tnconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`]'s type-level unsigned/positive/negative (depending on what is the prefix-sign) integer (i.e. the type that implements the [`typenum::Unsigned`]/[`typenum::Integer`] trait).
///
/// Because [`tnconst!`] can be evaluated into [`typenum::UInt`], [`typenum::PInt`] and [`typenum::NInt`], to disambiguate them, one is required to invoke the macro with either `+`, `-` to get [`tnconst!`] to evaluate the macro input as [`typenum::PInt`] or [`typenum::NInt`], respectively. [`typenum::UInt`] is the default and does not require any sign to be prefixed.
///
/// Examples:
///
/// If you invoke e.g. `tnconst![123]`, i.e. there is no sign prefixing the literal integer `123`, then this will be evaluated as a [`typenum::UInt`], specifically, [`typenum::U123`].
///
/// If you invoke e.g. `tnconst![-123]`, i.e. there is a negative sign prefixing the literal integer `123`, then this will be evaluated as a [`typenum::NInt`], specifically, [`typenum::N123`].
///
/// There are three ways you can invoke this macro.
///
/// ## 1. Invoke it with a literal integer
///
/// ```rust
/// use typenum::{N123, assert_type_eq};
/// use typenum_consts::tnconst;
///
/// type A = tnconst![-123];
/// assert_type_eq!(A, N123);
/// ```
///
/// ## 2. Invoke using an expression or many simple mathematical expressions
///
/// ```rust
/// use typenum::{P15, assert_type_eq};
/// use typenum_consts::tnconst;
/// type D = tnconst![+ {
///     a = 10;
///     b = 5;
///     a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, P15
/// }];
/// assert_type_eq!(D, P15);
/// ```
///
/// It is a compilation error if the mathematical expressions evaluate to a negative (positive) literal integer and you had specify a `+` (`-`) prefix to ask [`tnconst!`] to evaluate the input as a [`typenum::PInt`] ([`typenum::NInt`]).
///
/// ```compile_fail
/// use typenum::{P15, assert_type_eq};
/// use typenum_consts::tnconst;
/// type D = tnconst![+ {
///     a = 10;
///     b = 5;
///     b - a; // 5 - 10 = -5, cannot be made into a `P15`
/// }];
/// assert_type_eq!(D, P15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
///
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `tnconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`, `"ENV_VAR"` is the key to read the value from and `"./.env.prod"` is the file path relative to [`CARGO_MANIFEST_DIR`].
///
/// ```rust
/// use typenum::{U69, assert_type_eq};
/// use typenum_consts::tnconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type E = tnconst![env!("ENV_VAR");];
/// assert_type_eq!(E, U69);
/// ```
///
/// It is a compilation error if the environment variable evaluate to a negative (positive) literal integer and you had specify a `+` (`-`) prefix to ask [`tnconst!`] to evaluate the input as a [`typenum::PInt`] ([`typenum::NInt`]).
///
/// ```compile_fail
/// use typenum::{P69, assert_type_eq};
/// use typenum_consts::tnconst;
/// // ``` .env
/// // NENV_VAR=-69
/// // ```
/// type F = tnconst![+ env!("NENV_VAR");];
/// assert_type_eq!(F, P69);
/// ```
///
/// [`CARGO_MANIFEST_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
/// [`typenum::Unsigned`]: https://docs.rs/typenum/latest/typenum/marker_traits/trait.Unsigned.html
/// [`typenum::Integer`]: https://docs.rs/typenum/latest/typenum/marker_traits/trait.Integer.html
/// [`typenum::PInt`]: https://docs.rs/typenum/latest/typenum/int/struct.PInt.html
/// [`typenum::UInt`]: https://docs.rs/typenum/latest/typenum/uint/struct.UInt.html
/// [`typenum::NInt`]: https://docs.rs/typenum/latest/typenum/int/struct.NInt.html
/// [`typenum`]: https://github.com/paholg/typenum/tree/main/src
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

/// [`pconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`]'s type-level positive integer (i.e. the type that implements the [`typenum::Integer`] trait).
///
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
/// Compilation fails if the literal integer is prefixed with either a `-` or a `+`.
///
/// ```compile_fail
/// # use typenum::{P123, assert_type_eq};
/// # use typenum_consts::pconst;
/// type B = pconst![+123]; // Fail to compile
/// ```
///
/// ```compile_fail
/// # use typenum::{P123, assert_type_eq};
/// # use typenum_consts::pconst;
/// type C = pconst![-123]; // Fail to compile
/// ```
///
/// ## 2. Invoke using an expression or many simple mathematical expressions
///
/// ```rust
/// use typenum::{P15, assert_type_eq};
/// use typenum_consts::pconst;
/// type D = pconst![{
///     a = 10;
///     b = 5;
///     a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, P15
/// }];
/// assert_type_eq!(D, P15);
/// ```
///
/// It is a compilation error if the mathematical expressions evaluate to a negative literal integer.
///
/// ```compile_fail
/// use typenum::{P15, assert_type_eq};
/// use typenum_consts::pconst;
/// type D = pconst![{
///     a = 10;
///     b = 5;
///     b - a; // 5 - 10 = -5, cannot be made into a `PInt<U15>`
/// }];
/// assert_type_eq!(D, P15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
///
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `pconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`, `"ENV_VAR"` is the key to read the value from and `"./.env.prod"` is the file path relative to [`CARGO_MANIFEST_DIR`].
///
/// ```rust
/// use typenum::{P69, assert_type_eq};
/// use typenum_consts::pconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type E = pconst![env!("ENV_VAR");];
/// assert_type_eq!(E, P69);
/// ```
///
/// It is a compilation error if the environment variable evaluate to a negative literal integer.
///
/// ```compile_fail
/// use typenum::{P69, assert_type_eq};
/// use typenum_consts::pconst;
/// // ``` .env
/// // NENV_VAR=-69
/// // ```
/// type F = pconst![env!("NENV_VAR");];
/// assert_type_eq!(F, P69);
/// ```
///
/// [`CARGO_MANIFEST_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
/// [`typenum::Integer`]: https://docs.rs/typenum/latest/typenum/marker_traits/trait.Integer.html
/// [`typenum`]: https://github.com/paholg/typenum/tree/main/src
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

/// [`nconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`]'s type-level negative integer (i.e. the type that implements the [`typenum::Integer`] trait).
///
/// There are three ways you can invoke this macro.
///
/// ## 1. Invoke it with a literal integer
///
/// ```rust
/// use typenum::{N123, assert_type_eq};
/// use typenum_consts::nconst;
///
/// type A = nconst![123];
/// assert_type_eq!(A, N123);
/// ```
///
/// Compilation fails if the literal integer is prefixed with either a `-` or a `+`.
///
/// ```compile_fail
/// # use typenum::{N123, assert_type_eq};
/// # use typenum_consts::nconst;
/// type B = nconst![+123]; // Fail to compile
/// ```
///
/// ```compile_fail
/// # use typenum::{N123, assert_type_eq};
/// # use typenum_consts::nconst;
/// type C = nconst![-123]; // Fail to compile
/// ```
///
/// ## 2. Invoke using an expression or many simple mathematical expressions
///
/// ```rust
/// use typenum::{N15, assert_type_eq};
/// use typenum_consts::nconst;
/// type D = nconst![{
///     a = 10;
///     b = 5;
///     0 - a - b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, N15
/// }];
/// assert_type_eq!(D, N15);
/// ```
///
/// It is a compilation error if the mathematical expressions evaluate to a positive literal integer.
///
/// ```compile_fail
/// use typenum::{N15, assert_type_eq};
/// use typenum_consts::nconst;
/// type D = nconst![{
///     a = 10;
///     b = 5;
///     b + a; // 5 + 10 = 15, cannot be made into a `N15`
/// }];
/// assert_type_eq!(D, N15);
/// ```
///
/// ## 3. Invoke by reading from an environment variable
///
/// Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `nconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`, `"ENV_VAR"` is the key to read the value from and `"./.env.prod"` is the file path relative to [`CARGO_MANIFEST_DIR`].
///
/// ```rust
/// use typenum::{N69, assert_type_eq};
/// use typenum_consts::nconst;
/// // ``` .env
/// // NENV_VAR=-69
/// // ```
/// type E = nconst![env!("NENV_VAR");];
/// assert_type_eq!(E, N69);
/// ```
///
/// It is a compilation error if the environment variable evaluate to a positive literal integer.
///
/// ```compile_fail
/// use typenum::{N69, assert_type_eq};
/// use typenum_consts::nconst;
/// // ``` .env
/// // ENV_VAR=69
/// // ```
/// type F = nconst![env!("ENV_VAR");];
/// assert_type_eq!(F, N69);
/// ```
///
/// [`CARGO_MANIFEST_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
/// [`typenum::Integer`]: https://docs.rs/typenum/latest/typenum/marker_traits/trait.Integer.html
/// [`typenum`]: https://github.com/paholg/typenum/tree/main/src
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
