use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Error, LitInt, Result};
const HIGHEST: u64 = 1024;
fn uints() -> impl Iterator<Item = u64> {
    // Use hardcoded values to avoid issues with cross-compilation.
    // See https://github.com/paholg/typenum/issues/162
    let first2: u32 = 11; // (highest as f64).log(2.0).round() as u32 + 1;
    let first10: u32 = 4; // (highest as f64).log(10.0) as u32 + 1;
    (0..(HIGHEST + 1))
        .chain((first2..64).map(|i| 2u64.pow(i)))
        .chain((first10..20).map(|i| 10u64.pow(i)))
}

fn digits_to_uint(digits: &str) -> Result<proc_macro2::TokenStream> {
    let last_digit_ident = {
        let last_digit = digits.chars().last().ok_or_else(
            || {
                Error::new(Span::call_site(), format!("unable to get the last digit of the literal integer passed into `uconst` = {:?}", digits))
            }
        )?;
        format_ident!("U{}", last_digit)
    };
    let mut token_stream = quote!(::typenum::consts::#last_digit_ident);

    for (expo, d) in digits.chars().rev().enumerate() {
        eprintln!("`expo` = {expo}; `d` = {d}");
        let d_uint_ident = format_ident!("U{}", d);
        let expo_uint_ident = format_ident!("U{}", expo);
        token_stream = quote! {
            ::typenum::Sum::<
                ::typenum::Prod<
                    ::typenum::Exp::<
                        ::typenum::consts::U10,
                        ::typenum::consts::#expo_uint_ident
                        >,
                        ::typenum::consts::#d_uint_ident
                    >,
                #token_stream
            >
        };
        eprintln!("`token_stream` = {token_stream}");
    }

    Ok(token_stream)
}

fn can_represent_as_u32_or_u64(digits: &str) -> bool {
    // Check if the string is empty
    if digits.is_empty() {
        return false;
    }

    for ch in digits.chars() {
        // Check if the character is not a digit
        if !ch.is_digit(10) {
            return false;
        }
    }

    if digits.len() > 10 && digits[..digits.len() - 10].chars().any(|c| c != '0') {
        return false;
    }

    if digits.len() == 10 {
        return digits > format!("{:?}", U32OrU64::MAX).as_str();
    }

    true
}

#[cfg(target_pointer_width = "32")]
type U32OrU64 = u32;

#[cfg(target_pointer_width = "64")]
type U32OrU64 = u64;

pub(crate) fn uconst_impl(litint: LitInt) -> Result<TokenStream2> {
    eprintln!("`litint` = {litint}");

    if !can_represent_as_u32_or_u64(litint.base10_digits()) {
        return Err(Error::new(litint.span(), format!("the integer literal = `{litint:?}` passed into `uconst` is too large to be represented by {} bits", usize::BITS)));
    }

    let litint_u32_or_u64: U32OrU64 = match litint.base10_parse::<U32OrU64>() {
        Ok(num) => num,
        Err(err) => {
            return {
                let not_digits = litint
                    .base10_digits()
                    .chars()
                    .filter(|c| !c.is_digit(10))
                    .collect::<Vec<char>>()
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                Err(Error::new_spanned(
                    litint.token(),
                    format!(
                        "the literal passed into `uconst!(...)` cannot be parsed into `u64`; `{litint}` contains these characters which are not digits: {not_digits:?}; source error: {err}"
                    ),
                ))
            }
        }
    };
    if uints().any(|uint| uint == litint_u32_or_u64) {
        let ident = format_ident!("U{litint_u32_or_u64}");
        return Ok(quote!(::typenum::consts::#ident));
    }
    let ts = digits_to_uint(format!("{}", litint_u32_or_u64).as_str())?;
    Ok(quote!(#ts))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_can_represent_as_u32_or_u64() {
        assert!(can_represent_as_u32_or_u64("123"));
        assert!(can_represent_as_u32_or_u64("4294967295"));
        assert!(can_represent_as_u32_or_u64("0"));
        assert!(can_represent_as_u32_or_u64("1"));
        assert!(can_represent_as_u32_or_u64("9999999999"));
        assert!(can_represent_as_u32_or_u64("0123")); // Leading zeros
        assert!(can_represent_as_u32_or_u64("000"));
        assert!(can_represent_as_u32_or_u64("0000000000001234")); // Leading zeros

        assert!(!can_represent_as_u32_or_u64(""));
        assert!(!can_represent_as_u32_or_u64("abc"));
        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("4294967296"));
        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("10000000000")); // Exceeds u32::MAX
        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("100000000000")); // Exceeds u32::MAX
    }
}
