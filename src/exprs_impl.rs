use crate::types::{LitInteger, MathExprs};
use proc_macro2::Span;
use quote::ToTokens;
use rsc::{parse, tokenize, Interpreter, Variant};
use syn::{spanned::Spanned, Error, LitInt, Result, Stmt};

const ANS: &str = "ANS";

pub(crate) fn eval_exprs(math_exprs: MathExprs) -> Result<LitInteger> {
    match math_exprs {
        MathExprs::Negative(stmts) => eval_exprs_neg(stmts),
        MathExprs::Positive(stmts) => eval_exprs_pos(stmts),
        MathExprs::Unsigned(stmts) => eval_exprs_usig(stmts),
    }
}

fn interpreter_eval(math_stmts: &[Stmt]) -> Result<isize> {
    let mut interpreter = Interpreter::new();
    interpreter.set_var(ANS.into(), Variant::Num(0));
    if math_stmts.is_empty() {
        let last_stmt = get_last_stmt(math_stmts)?;
        return Err(Error::new(
            last_stmt.span(),
            "the mathematical expressions provided into the macro cannot be empty",
        ));
    }
    let mut final_ans: Option<isize> = None;
    for math_stmt in math_stmts {
        let mut ts_string = math_stmt.to_token_stream().to_string();
        if ts_string.ends_with(';') {
            ts_string = ts_string[..ts_string.len() - 1].to_string();
        }
        let tokens = tokenize::<isize>(ts_string.as_str())
            .map_err(|err| Error::new(math_stmt.span(), format!("{err:?}")))?;
        let expr = parse::<isize>(&tokens)
            .map_err(|err| Error::new(math_stmt.span(), format!("{err:?}")))?;
        let ans = interpreter
            .eval(&expr)
            .map_err(|err| Error::new(math_stmt.span(), format!("{err:?}")))?;
        interpreter.set_var(ANS.into(), Variant::Num(ans));
        final_ans = Some(ans);
    }
    if let Some(final_ans) = final_ans {
        Ok(final_ans)
    } else {
        Err(Error::new(
            Span::call_site(),
            "the mathematical expressions passed into the procedural macro should not be empty",
        ))
    }
}

fn eval_exprs_neg(math_stmts: Vec<Stmt>) -> Result<LitInteger> {
    let mut final_ans = interpreter_eval(&math_stmts)?;
    let last_stmt = get_last_stmt(&math_stmts)?;
    if final_ans <= 0 {
        final_ans = -final_ans;
    };
    Ok(LitInteger::Negative {
        lit_integer: LitInt::new(format! {"{}", final_ans}.as_str(), last_stmt.span()),
    })
}

fn get_last_stmt(math_stmts: &[Stmt]) -> Result<&Stmt> {
    math_stmts.last().ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "cannot get the last statement from `math_stmts`",
        )
    })
}

fn eval_exprs_pos(math_stmts: Vec<Stmt>) -> Result<LitInteger> {
    let final_ans = interpreter_eval(&math_stmts)?;
    let last_stmt = get_last_stmt(&math_stmts)?;
    if final_ans <= 0 {
        return Err(Error::new(
            last_stmt.span(),
            format!(
                "expressions should be evaluated to a positive integer, got `{final_ans}` instead"
            ),
        ));
    };
    Ok(LitInteger::Positive {
        lit_integer: LitInt::new(format! {"{}", final_ans}.as_str(), last_stmt.span()),
    })
}

fn eval_exprs_usig(math_stmts: Vec<Stmt>) -> Result<LitInteger> {
    let final_ans = interpreter_eval(&math_stmts)?;
    let last_stmt = get_last_stmt(&math_stmts)?;
    if final_ans < 0 {
        return Err(Error::new(
            last_stmt.span(),
            format!("expressions should be evaluated to a non-negative or positive integer, got `{final_ans}` instead")
        ));
    };
    Ok(LitInteger::Unsigned {
        lit_integer: LitInt::new(format! {"{}", final_ans}.as_str(), last_stmt.span()),
    })
}
