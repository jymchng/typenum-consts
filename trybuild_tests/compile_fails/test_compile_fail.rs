#[cfg(target_pointer_width = "32")]
type I32OrI64 = i32;
#[cfg(target_pointer_width = "64")]
type I32OrI64 = i64;

fn test_tnconst() {
    use typenum_consts::tnconst;

    type A = tnconst![+ env!("BAD_SECRET");];
    type B = tnconst![+ env!("BAD_SECRET", "");];
    type C = tnconst![+ env!("BAD_SECRET",,);];
    type D = tnconst![+ env!(,,"BAD_SECRET");];
    type E = tnconst![+ env!("", );];
    type F = tnconst![+ env!("", "");];
    type G = tnconst![% env!("", "");];
    type H = tnconst![{
        x = 69;
        lame;
    }];
    type I = tnconst![-{
        a = 5;
        b = 10;
        a + b
    }];
    type J = tnconst![+ {
        a = 5;
        b = 10;
        a - b
    }];
    type K = tnconst![{
        a = 5;
        b = 10;
        a - b
    }];
    type L = tnconst![- env!("BAD_SECRET");];
    type M = tnconst![- env!("BAD_SECRET", "");];
    type N = tnconst![- env!("BAD_SECRET",,);];
    type O = tnconst![- env!(,,"BAD_SECRET");];
    type P = tnconst![- env!("", );];
    type Q = tnconst![- env!("", "");];
    type R = tnconst![- env!("ENV_VAR");];
    type S = tnconst![+ env!("NENV_VAR");];
    type T = tnconst![ env!("NENV_VAR");];
    type U = tnconst![ ggenv!("NENV_VAR");];
    println!("test_tnconst Passed!");
}

fn test_pconst() {
    use typenum_consts::pconst;

    type A = pconst![ env!("BAD_SECRET");];
    type B = pconst![ env!("BAD_SECRET", "");];
    type C = pconst![ env!("BAD_SECRET",,);];
    type D = pconst![ env!(,,"BAD_SECRET");];
    type E = pconst![ env!("", );];
    type F = pconst![ env!("", "");];
    type G = pconst![% env!("", "");];
    type H = pconst![ 6969;];
    type J = pconst![ 6969 {};];
    type K = pconst![ 6969 {}];
    type I = pconst![{
        x = 69;
        lame;
    }];
    type M = pconst![+ 69];
    type N = pconst![+ env!("ENV_VAR");];
    type O = pconst![+ {
        a = 5;
        b = 10;
        a + b
    }];
    type P = pconst![{
        a = 5;
        b = 10;
        a - b
    }];

    println!("test_pconst Passed!");
}

fn test_nconst() {
    use typenum_consts::nconst;

    type A = nconst![ env!("BAD_SECRET");];
    type B = nconst![ env!("BAD_SECRET", "");];
    type C = nconst![ env!("BAD_SECRET",,);];
    type D = nconst![ env!(,,"BAD_SECRET");];
    type E = nconst![ env!("", );];
    type F = nconst![ env!("", "");];
    type G = nconst![% env!("", "");];
    type H = nconst![ 6969;];
    type J = nconst![ 6969 {};];
    type K = nconst![ 6969 {}];
    type I = nconst![{
        x = 69;
        lame;
    }];
    type M = nconst![{}];
    type R = nconst![-69];
    type N = nconst![- env!("ENV_VAR");];
    type O = nconst![-{
        a = 5;
        b = 10;
        a + b
    }];
    type P = nconst![{
        a = 5;
        b = 10;
        a + b
    }];

    println!("test_nconst Passed!");
}

fn test_uconst() {
    use typenum_consts::uconst;

    type A = uconst![ env!("BAD_SECRET");];
    type B = uconst![ env!("BAD_SECRET", "");];
    type C = uconst![ env!("BAD_SECRET",,);];
    type D = uconst![ env!(,,"BAD_SECRET");];
    type E = uconst![ env!("", );];
    type F = uconst![ env!("", "");];
    type G = uconst![% env!("", "");];
    type H = uconst![ 6969;];
    type J = uconst![ 6969 {};];
    type K = uconst![ 6969 {}];
    type I = uconst![{
        x = 69;
        lame;
    }];
    type N = uconst![6969;];
    type M = uconst![6969;];
    type O = uconst![env!("ENV_VAR");;];
    type P = uconst![ {
        100 - 6969;
    };];
    type R = uconst![-69];
    type S = uconst![- env!("ENV_VAR");];
    type T = uconst![-{
        a = 5;
        b = 10;
        a + b
    }];
    type X = uconst![-{
        a = 5;
        b = 10;
        a - b
    }];
    type U = uconst![+ 69];
    type V = uconst![+ env!("ENV_VAR");];
    type W = uconst![+ {
        a = 5;
        b = 10;
        a + b
    }];
    type Y = uconst![+ {
        a = 5;
        b = 10;
        a - b
    }];
    println!("test_uconst Passed!");
}

fn main() {
    test_tnconst();
    test_pconst();
    test_nconst();
    test_uconst();
    test_nconst_math_exprs_positive_result_no_sign();
    test_env_give_positive_int_nconst();
}

fn test_nconst_math_exprs_positive_result_no_sign() {
    use typenum::{assert_type_eq, N15};
    use typenum_consts::nconst;
    type D = nconst![{
        a = 10;
        b = 5;
        b + a; // Last statement is always the final returned value to be casted into `typenum` type-level integer, U15
    }];
    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <D as typenum::ToInt<I32OrI64>>::INT,
        <N15 as typenum::ToInt<I32OrI64>>::INT,
    );
    assert_type_eq!(D, N15);
}

fn test_env_give_positive_int_nconst() {
    use typenum::{assert_type_eq, N69};
    use typenum_consts::nconst;
    type D = nconst![env!("ENV_VAR");];
    assert_type_eq!(D, N69);

    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <D as typenum::ToInt<I32OrI64>>::INT,
        <N69 as typenum::ToInt<I32OrI64>>::INT,
    );
}

fn test_env_give_negative_int_pconst() {
    use typenum::{assert_type_eq, P69};
    use typenum_consts::pconst;
    type D = pconst![env!("NENV_VAR");];
    assert_type_eq!(D, P69);

    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <D as typenum::ToInt<I32OrI64>>::INT,
        <P69 as typenum::ToInt<I32OrI64>>::INT,
    );
}

fn test_env_give_negative_int_uconst() {
    use typenum::{assert_type_eq, U69};
    use typenum_consts::uconst;
    type D = uconst![env!("NENV_VAR");];
    assert_type_eq!(D, U69);

    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <D as typenum::ToInt<I32OrI64>>::INT,
        <U69 as typenum::ToInt<I32OrI64>>::INT,
    );
}
