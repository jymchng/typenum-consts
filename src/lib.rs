#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use crate::macros::debug_eprintln;
use crate::types::LitInteger;
use proc_macro::{self, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Attribute, Error, Ident, LitInt, Meta, Token,
};

mod macros;
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

#[derive(Debug)]
struct OuterAttributesAndValue {
    attrs: Vec<Attribute>,
    value: LitInteger,
}

// impl OuterAttributesAndValue {

//     fn to_outer_attributes_tokenstream2(&self, f: fn(LitInteger) -> TokenStream2) -> OuterAttributesAndTokenStream2 {
//         OuterAttributesAndTokenStream2 {
//             attrs: self.attrs.clone(),
//             token_stream: f(self.value.clone()),
//         }
//     }
// }

// #[derive(Debug)]
// struct OuterAttributesAndTokenStream2 {
//     attrs: Vec<Attribute>,
//     token_stream: TokenStream2,
// }

// impl OuterAttributesAndTokenStream2 {

//     fn to_token_stream2(&self) -> TokenStream2 {
//         let attrs = self.attrs.clone();
//         let value = self.token_stream.clone();
//         quote!(#(#attrs)* #value)
//     }
// }

impl Parse for OuterAttributesAndValue {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        debug_eprintln!("`input` = {input:?}");
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct OuterAttributesAndValues {
    attrs_values: Vec<OuterAttributesAndValue>,
}

impl Parse for OuterAttributesAndValues {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut v: Vec<OuterAttributesAndValue> = vec![];

        loop {
            if input.is_empty() {
                break;
            }

            v.push(input.parse::<OuterAttributesAndValue>()?);

            if input.is_empty() {
                break;
            }
        }

        Ok(Self { attrs_values: v })
    }
}

// // fn handle_outer_attrs_values(outer_attrs_values: OuterAttributesAndValues) -> Result<LitInteger> {
// //     let _ = outer_attrs_values.attrs_values
// //             .iter()
// //             .map(
// //                 |outer_attrs_value|
// //             )

// // }

type ProcMacroFn = Box<dyn Fn(TokenStream, TokenStream) -> TokenStream>;

// fn handle_outer_attrs_value(
//     outer_attrs_value: OuterAttributesAndValue,
// ) -> Result<Option<LitInteger>, Error> {
//     let OuterAttributesAndValue {
//         attrs,
//         value: orig_value,
//     } = outer_attrs_value;
//     let mut flag: u8;
//     let mut orig_value = match orig_value {
//         LitInteger::Negative { lit_integer } => {
//             flag = 1;
//             lit_integer
//         }
//         LitInteger::Positive { lit_integer } => {
//             flag = 2;
//             lit_integer
//         }
//         LitInteger::Unsigned { lit_integer } => {
//             flag = 3;
//             lit_integer
//         }
//     };
//     let mut value = Option::Some(orig_value.clone());
//     let mut v = Vec::<(ProcMacroFn, LitInteger)>::new();
//     let mut tokens: TokenStream;
//     for attr in attrs {
//         let Some(litint) = syn::parse::<LitInteger>(tokens).ok() else {
//             break;
//         };
//         let litint = match litint {
//             LitInteger::Negative { lit_integer } => {
//                 lit_integer
//             }
//             LitInteger::Positive { lit_integer } => {
//                 lit_integer
//             }
//             LitInteger::Unsigned { lit_integer } => {
//                 lit_integer
//             }
//         };
//         let proc_macro_fn = core::cfg as ProcMacroFn;
//         let Meta::List(meta_list) = attr.meta else {
//             return Err(Error::new(
//                 attr.span(),
//                 "Attributes requires a metadata parameters",
//             ));
//         };
//         tokens = proc_macro_fn(meta_list.tokens.to_token_stream().into(), litint.into());
//     }
//     if let Some(value) = value {
//         match flag {
//             1 => Ok(Some(LitInteger::Negative { lit_integer: value })),
//             2 => Ok(Some(LitInteger::Positive { lit_integer: value })),
//             3 => Ok(Some(LitInteger::Unsigned { lit_integer: value })),
//         }
//     } else {
//         Ok(None)
//     }

// }

#[proc_macro]
pub fn tnconst(items: TokenStream) -> TokenStream {
    debug_eprintln!("`items`: {:?}", items);
    if let Ok(lit_integer) = syn::parse::<LitInteger>(items.clone()) {
        tnconst_impl::tnconst_impl(lit_integer)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    } else if let Ok(attribute) = syn::parse::<OuterAttributesAndValues>(items) {
        debug_eprintln!("`attribute`: {:?}", attribute);
        TokenStream::new()
    } else {
        Error::new(
            Span::call_site(),
            "the syntax passed into `tnconst` is not yet supported",
        )
        .into_compile_error()
        .into()
    }
}

#[proc_macro]
pub fn pconst(items: TokenStream) -> TokenStream {
    let lit_integer: LitInteger = parse_macro_input!(items as LitInteger);
    let result = match lit_integer {
        LitInteger::Unsigned { lit_integer } => {
            Err(
                Error::new(
                    lit_integer.span(),
                    "the literal passed into `tnconst!` or `pconst!` needs to have a `+` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                ))
        }
        LitInteger::Positive { lit_integer } => tnconst_impl::pconst_impl(lit_integer),
        LitInteger::Negative { lit_integer } => {
            Err(
                Error::new(
                    lit_integer.span(),
                    "the literal passed into `tnconst!` or `pconst!` needs to have a `+` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                ))
        }
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

#[proc_macro]
pub fn nconst(items: TokenStream) -> TokenStream {
    let lit_integer: LitInteger = parse_macro_input!(items as LitInteger);
    let result = match lit_integer {
        LitInteger::Unsigned { lit_integer } => {
            Err(
                Error::new(
                    lit_integer.span(),
                    "the literal passed into `tnconst!` or `nconst!` needs to have a `-` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                ))
        }
        LitInteger::Positive { lit_integer } => {
            Err(
                Error::new(
                    lit_integer.span(),
                    "the literal passed into `tnconst!` or `nconst!` needs to have a `-` character at the beginning, it does not have it; literal passed in is: {lit_integer:?}",
                ))
        }
        LitInteger::Negative { lit_integer } => tnconst_impl::nconst_impl(lit_integer),
    };
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}
