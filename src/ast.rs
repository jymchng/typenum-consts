use crate::{ast_macro::AllowedMacros, debug_eprintln};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Brace,
    Block, Ident, LitInt, Macro, Result, Stmt, Token,
};

pub(crate) enum MathExprs {
    Unsigned(Vec<Stmt>),
    Positive(Vec<Stmt>),
    Negative(Vec<Stmt>),
}

enum Sign {
    P,
    N,
    U,
}

impl Sign {
    fn math_exprs(&self, stmts: Vec<Stmt>) -> Result<LitIntegerOrExprs> {
        match self {
            Self::P => Ok(LitIntegerOrExprs::Exprs(MathExprs::Positive(stmts))),
            Self::N => Ok(LitIntegerOrExprs::Exprs(MathExprs::Negative(stmts))),
            Self::U => Ok(LitIntegerOrExprs::Exprs(MathExprs::Unsigned(stmts))),
        }
    }

    fn lit_integer(&self, litint: LitInt) -> Result<LitIntegerOrExprs> {
        match self {
            Self::P => Ok(LitIntegerOrExprs::LitInteger(LitInteger::Positive {
                lit_integer: litint,
            })),
            Self::N => Ok(LitIntegerOrExprs::LitInteger(LitInteger::Negative {
                lit_integer: litint,
            })),
            Self::U => Ok(LitIntegerOrExprs::LitInteger(LitInteger::Unsigned {
                lit_integer: litint,
            })),
        }
    }
}

fn which_lit_integer_or_exprs(input: ParseStream, sign: Sign) -> Result<LitIntegerOrExprs> {
    let lookahead = input.lookahead1();
    let result = if lookahead.peek(Brace) {
        debug_eprintln!(
            "`input` = {input}, `lookahead` = {}",
            lookahead.error().to_string()
        );
        let content;
        braced!(content in input);
        if content.is_empty() {
            return Err(content
                .error("the content within the block delimited by `{...}` must not be empty"));
        }
        let stmts = content.call(Block::parse_within)?;
        Ok(sign.math_exprs(stmts)?)
    } else if lookahead.peek(Ident) {
        let some_macro = input.parse::<Macro>()?;
        let allowed_macro = AllowedMacros::which_macro(&some_macro)?;
        let litint = LitInt::new(&allowed_macro.invoke_macro()?, some_macro.span());
        Ok(sign.lit_integer(litint)?)
    } else {
        Ok(sign.lit_integer(input.parse::<LitInt>()?)?)
    };
    if !input.is_empty() {
        let _ = input.parse::<Token![;]>()?;
    };
    result
}

impl Parse for LitIntegerOrExprs {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            let _ = input.parse::<Token![+]>();
            debug_eprintln!("`input` = {input}");
            Ok(which_lit_integer_or_exprs(input, Sign::P)?)
        } else if lookahead.peek(Token![-]) {
            let _ = input.parse::<Token![-]>();
            debug_eprintln!("`input` = {input}");
            Ok(which_lit_integer_or_exprs(input, Sign::N)?)
        } else {
            debug_eprintln!("`input` = {input}");
            Ok(which_lit_integer_or_exprs(input, Sign::U)?)
        }
    }
}

pub(crate) enum LitIntegerOrExprs {
    Exprs(MathExprs),
    LitInteger(LitInteger),
}

pub(crate) enum LitInteger {
    Positive { lit_integer: LitInt },
    Negative { lit_integer: LitInt },
    Unsigned { lit_integer: LitInt },
}

impl Parse for LitInteger {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            let _ = input.parse::<Token![+]>();
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::Positive { lit_integer })
        } else if lookahead.peek(Token![-]) {
            let _ = input.parse::<Token![-]>();
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::Negative { lit_integer })
        } else {
            let lit_integer: LitInt = input.parse()?;
            debug_eprintln!("{:?}", lit_integer);
            Ok(LitInteger::Unsigned { lit_integer })
        }
    }
}
