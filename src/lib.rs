#![no_std]

use crate::macros::{debug_eprintln, no_std_format};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, LitInt};

mod macros;
mod no_std_formatter;
mod uconst_impl;

#[proc_macro]
pub fn uconst(items: TokenStream) -> TokenStream {
    let litint = parse_macro_input!(items as LitInt);
    debug_eprintln!("{}", litint);
    uconst_impl::uconst_impl(litint)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn tnconst(attrs: TokenStream, items: TokenStream) -> TokenStream {
    debug_eprintln!("`attrs` = {attrs}, `items` = {items}");
    use syn::Item;
    let item = parse_macro_input!(items as Item);
    let ts = quote! {
        struct Wrapper2<T: ::typenum::Unsigned> {
            inner: PhantomData<T>,
        }
    };
    match item {
        Item::Struct(struct_) => {
            debug_eprintln!("`struct_` = {struct_:?}");
            ts.clone()
        }
        Item::Enum(enum_) => {
            debug_eprintln!("`enum_` = {enum_:?}");
            ts.clone()
        }
        unsupported => {
            syn::Error::new_spanned(unsupported, "#[pyclass] only supports structs and enums.")
                .into_compile_error()
                .into()
        }
    };
    ts.into()
}
