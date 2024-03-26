fn test_tnconst() {
    use typenum_consts::tnconst;

    type A = tnconst![+ env!("SEXY_SECRET");];
    type B = tnconst![+ env!("SEXY_SECRET", "");];
    type C = tnconst![+ env!("SEXY_SECRET",,);];
    type D = tnconst![+ env!(,,"SEXY_SECRET");];
    type E = tnconst![+ env!("", );];
    type F = tnconst![+ env!("", "");];
    type G = tnconst![% env!("", "");];
    type H = tnconst![{
        x = 69;
        lame;
    }];
    println!("test_tnconst Passed!");
}

fn test_pconst() {
    use typenum_consts::pconst;

    type A = pconst![+ env!("SEXY_SECRET");];
    type B = pconst![+ env!("SEXY_SECRET", "");];
    type C = pconst![+ env!("SEXY_SECRET",,);];
    type D = pconst![+ env!(,,"SEXY_SECRET");];
    type E = pconst![+ env!("", );];
    type F = pconst![+ env!("", "");];
    type G = pconst![% env!("", "");];
    type H = pconst![+ 6969;];
    type J = pconst![+ 6969 {};];
    type K = pconst![+ 6969 {}];
    type L = pconst![+ 0 ];
    assert_eq!(<L as typenum::Unsigned>::USIZE, 0);
    type I = pconst![{
        x = 69;
        lame;
    }];
    println!("test_pconst Passed!");
}

fn test_nconst() {
    use typenum_consts::nconst;

    type A = nconst![- env!("SEXY_SECRET");];
    type B = nconst![- env!("SEXY_SECRET", "");];
    type C = nconst![- env!("SEXY_SECRET",,);];
    type D = nconst![- env!(,,"SEXY_SECRET");];
    type E = nconst![- env!("", );];
    type F = nconst![- env!("", "");];
    type G = nconst![% env!("", "");];
    type H = nconst![- 6969;];
    type J = nconst![- 6969 {};];
    type K = nconst![- 6969 {}];
    type L = nconst![-0];
    assert_eq!(<L as typenum::Unsigned>::USIZE, 0);
    type I = nconst![{
        x = 69;
        lame;
    }];
    type M = nconst![-{}];
    println!("test_nconst Passed!");
}

fn test_uconst() {
    use typenum_consts::uconst;

    type A = uconst![ env!("SEXY_SECRET");];
    type B = uconst![ env!("SEXY_SECRET", "");];
    type C = uconst![ env!("SEXY_SECRET",,);];
    type D = uconst![ env!(,,"SEXY_SECRET");];
    type E = uconst![ env!("", );];
    type F = uconst![ env!("", "");];
    type G = uconst![% env!("", "");];
    type H = uconst![ 6969;];
    type J = uconst![ 6969 {};];
    type K = uconst![ 6969 {}];
    type L = uconst![0];
    assert_eq!(<L as typenum::Unsigned>::USIZE, 0);
    type I = uconst![{
        x = 69;
        lame;
    }];
    type N = uconst![+ 6969;];
    type M = uconst![- 6969;];
    println!("test_uconst Passed!");
}

fn main() {
    test_tnconst();
    test_pconst();
    test_nconst();
    test_uconst();
}
