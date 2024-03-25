fn main_one() {
    use typenum_consts::tnconst;

    type A = tnconst![+ env!("SEXY_SECRET");];
    type B = tnconst![+ env!("SEXY_SECRET", "");];
    type C = tnconst![+ env!("SEXY_SECRET",,);];
    type D = tnconst![+ env!(,,"SEXY_SECRET");];
    type E = tnconst![+ env!("", );];
    type F = tnconst![+ env!("", "");];
    type G = tnconst![% env!("", "");];
    type H = tnconst![+ 6969;];
    type I = tnconst![{
        x = 69;
        lame;
    }];
}

fn main_two() {
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
}

fn main() {
    main_one();
    main_two();
}
