use crate::debug_eprintln;
use syn::{
    parse::{Parse, ParseStream}, LitInt, Result, Token,
};

pub enum LitInteger {
    PositiveInteger { lit_integer: LitInt },
    NegativeInteger { lit_integer: LitInt },
    UnsignedInteger { lit_integer: LitInt },
}

impl Parse for LitInteger {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            let _ = input.parse::<Token![+]>();
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::PositiveInteger { lit_integer })
        } else if lookahead.peek(Token![-]) {
            let _ = input.parse::<Token![-]>();
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::NegativeInteger { lit_integer })
        } else {
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::UnsignedInteger { lit_integer })
        }
    }
}
