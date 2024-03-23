use crate::macros::{debug_eprintln, no_std_format};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Error, LitInt, Result};

// Literally, copied from `typenum`.
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
    let mut token_stream = quote!(::typenum::consts::U0);

    for (expo, d) in digits.chars().rev().enumerate() {
        debug_eprintln!("`expo` = {expo}; `d` = {d}");
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
        debug_eprintln!("`token_stream` = {token_stream}");
    }

    Ok(token_stream)
}

fn can_represent_as_u32_or_u64(digits: &str) -> Result<bool> {
    #[cfg(target_pointer_width = "32")]
    let num_digit: u32 = 10;

    #[cfg(target_pointer_width = "64")]
    let num_digit: u32 = 20;

    if digits.is_empty() {
        return Ok(false);
    }

    for ch in digits.chars() {
        if !ch.is_ascii_digit() {
            return Ok(false);
        }
    }

    if digits.len() > num_digit as usize
        && digits[..digits.len() - num_digit as usize]
            .chars()
            .any(|c| c != '0')
    {
        return Ok(false);
    }

    if digits.len() == num_digit as usize {
        debug_eprintln!("`if digits.len() == num_digit as usize`: {digits}");
        #[cfg(target_pointer_width = "64")]
        let mut buf = [0u8; 20];

        #[cfg(target_pointer_width = "32")]
        let mut buf = [0u8; 10];

        return Ok(digits
            <= no_std_format!(buf, "{}", usize::MAX).map_err(|_| {
                Error::new(
                    Span::call_site(),
                    "unable to allocate enough memory to store decimal representation of `usize::MAX`",
                )
            })?);
    }

    Ok(true)
}

pub(crate) fn uconst_impl(litint: LitInt) -> Result<TokenStream2> {
    debug_eprintln!("`litint` = {litint}");

    if !can_represent_as_u32_or_u64(litint.base10_digits())? {
        let mut buf = [0_u8; 512];
        let err_msg = no_std_format!(buf, "the integer literal = `{litint:?}` passed into `uconst` is too large to be represented by {} bits", usize::BITS)
            .map_err(
                |_| Error::new(
                    litint.span(),
                    "unable to allocate enough memory for the error message, 512 bytes is insufficient; error originated from `can_represent_as_u32_or_u64`"
                )
        )?;
        return Err(Error::new(litint.span(), err_msg));
    }

    let litint_u32_or_u64: usize = match litint.base10_parse::<usize>() {
        Ok(num) => num,
        Err(err) => {
            return {
                let mut wrong_chars = [0_u8; 512];
                let _ = litint
                    .base10_digits()
                    .chars()
                    .filter(|c| !c.is_digit(10))
                    .enumerate()
                    .map(|(idx, c)| {
                        let wrong_chars_idx_mut_ref = wrong_chars.get_mut(idx).ok_or_else(
                            || Error::new(litint.span(),
                                "number of wrong characters in the literal constant passed into `uconst` is larger than 512 bytes in size", 
                            )
                        )?;
                        *wrong_chars_idx_mut_ref = c as u8;
                        Ok::<(), Error>(())
                    });
                let wrong_chars_str = core::str::from_utf8(&wrong_chars).or_else(|_| {
                    Err(Error::new(litint.span(),
                    "there are some characters passed into the `uconst` macro are not valid utf-8",
                ))
                })?;
                let mut buf = [0u8; 512];
                Err(Error::new_spanned(
                    litint.token(),
                    no_std_format!(
                        &mut buf,
                        "the literal passed into `uconst!(...)` cannot be parsed into `u64`; `{litint}` contains these characters which are not digits: {wrong_chars_str:?}; source error: {err}"
                    ).map_err(
                        |_| Error::new(litint.span(), "512 bytes is not sufficient memory space to format the error message; error originated from forming `wrong_chars_str`")
                    )?,
                ))
            }
        }
    };
    if uints().any(|uint| uint as usize == litint_u32_or_u64) {
        let ident = format_ident!("U{litint_u32_or_u64}");
        return Ok(quote!(::typenum::consts::#ident));
    }
    let mut buf = [0u8; 512];
    let ts = digits_to_uint(no_std_format!(&mut buf, "{}", litint_u32_or_u64).map_err(|_| {
        Error::new(Span::call_site(), "unable to allocate enough memory for the string representation of `litint_u32_or_u64`")
    })?)?;
    Ok(quote!(#ts))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_can_represent_as_u32_or_u64() {
        assert!(can_represent_as_u32_or_u64("123").unwrap());
        assert!(can_represent_as_u32_or_u64("4294967295").unwrap());
        assert!(can_represent_as_u32_or_u64("0").unwrap());
        assert!(can_represent_as_u32_or_u64("1").unwrap());
        assert!(can_represent_as_u32_or_u64("9999999999").unwrap());
        assert!(can_represent_as_u32_or_u64("0123").unwrap()); // Leading zeros
        assert!(can_represent_as_u32_or_u64("000").unwrap());
        assert!(can_represent_as_u32_or_u64("0000000000001234").unwrap()); // Leading zeros

        assert!(!can_represent_as_u32_or_u64("").unwrap());
        assert!(!can_represent_as_u32_or_u64("abc").unwrap());

        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("4294967296").unwrap());
        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("10000000000").unwrap()); // Exceeds u32::MAX
        #[cfg(target_pointer_width = "32")]
        assert!(!can_represent_as_u32_or_u64("100000000000").unwrap()); // Exceeds u32::MAX

        #[cfg(target_pointer_width = "64")]
        assert!(can_represent_as_u32_or_u64("18446744073709551615").unwrap()); // u64::MAX
        #[cfg(target_pointer_width = "64")]
        assert!(!can_represent_as_u32_or_u64("18446744073709551616").unwrap()); // Exceeds u64::MAX
    }
}
