use crate::{debug_eprintln, dotenv::EnvMacro};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket},
    Block, Ident, LitInt, Result, Stmt, Token,
};

pub(crate) enum MathExprs {
    Unsigned(Vec<Stmt>),
    Positive(Vec<Stmt>),
    Negative(Vec<Stmt>),
}

impl Parse for LitIntegerOrExprs {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            let _ = input.parse::<Token![+]>();
            let lookahead = input.lookahead1();
            debug_eprintln!("`input` = {input}");
            if lookahead.peek(Brace) || lookahead.peek(Bracket) {
                debug_eprintln!(
                    "`input` = {input}, `lookahead` = {}",
                    lookahead.error().to_string()
                );
                let content;
                braced!(content in input);
                let stmts = content.call(Block::parse_within)?;
                Ok(LitIntegerOrExprs::Exprs(MathExprs::Positive(stmts)))
            } else if lookahead.peek(Ident) {
                let env_var = input.parse::<EnvMacro>()?;
                return Ok(LitIntegerOrExprs::LitInteger(LitInteger::Positive {
                    lit_integer: LitInt::new(&env_var.read_env_value()?, env_var.key.span()),
                }));
            } else {
                Ok(LitIntegerOrExprs::LitInteger(LitInteger::Positive {
                    lit_integer: input.parse::<LitInt>()?,
                }))
            }
        } else if lookahead.peek(Token![-]) {
            let _ = input.parse::<Token![-]>();
            let lookahead = input.lookahead1();
            debug_eprintln!("`input` = {input}");
            if lookahead.peek(Brace) || lookahead.peek(Bracket) {
                debug_eprintln!(
                    "`input` = {input}, `lookahead` = {}",
                    lookahead.error().to_string()
                );
                let content;
                braced!(content in input);
                let stmts = content.call(Block::parse_within)?;
                Ok(LitIntegerOrExprs::Exprs(MathExprs::Negative(stmts)))
            } else if lookahead.peek(Ident) {
                let env_var = input.parse::<EnvMacro>()?;
                return Ok(LitIntegerOrExprs::LitInteger(LitInteger::Negative {
                    lit_integer: LitInt::new(&env_var.read_env_value()?, env_var.key.span()),
                }));
            } else {
                Ok(LitIntegerOrExprs::LitInteger(LitInteger::Negative {
                    lit_integer: input.parse::<LitInt>()?,
                }))
            }
        } else {
            let lookahead = input.lookahead1();
            debug_eprintln!("`input` = {input}");
            if lookahead.peek(Brace) || lookahead.peek(Bracket) {
                debug_eprintln!(
                    "`input` = {input}, `lookahead` = {}",
                    lookahead.error().to_string()
                );
                let content;
                braced!(content in input);
                let stmts = content.call(Block::parse_within)?;
                Ok(LitIntegerOrExprs::Exprs(MathExprs::Unsigned(stmts)))
            } else if lookahead.peek(Ident) {
                let env_var = input.parse::<EnvMacro>()?;
                return Ok(LitIntegerOrExprs::LitInteger(LitInteger::Unsigned {
                    lit_integer: LitInt::new(&env_var.read_env_value()?, env_var.key.span()),
                }));
            } else {
                Ok(LitIntegerOrExprs::LitInteger(LitInteger::Unsigned {
                    lit_integer: input.parse::<LitInt>()?,
                }))
            }
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
