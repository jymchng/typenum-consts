use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, LitInt};

mod uconst_impl;

#[proc_macro]
pub fn uconst(items: TokenStream) -> TokenStream {
    let litint = parse_macro_input!(items as LitInt);
    eprintln!("{}", litint);
    uconst_impl::uconst_impl(litint)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
