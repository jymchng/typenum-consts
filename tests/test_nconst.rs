#[test]
fn test_tnconst_four() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![-84938493];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::NInt<
        ::typenum::Sum<
            ::typenum::Prod<
                ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>,
                ::typenum::consts::U8,
            >,
            ::typenum::Sum<
                ::typenum::Prod<
                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>,
                    ::typenum::consts::U4,
                >,
                ::typenum::Sum<
                    ::typenum::Prod<
                        ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>,
                        ::typenum::consts::U9,
                    >,
                    ::typenum::Sum<
                        ::typenum::Prod<
                            ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>,
                            ::typenum::consts::U3,
                        >,
                        ::typenum::Sum<
                            ::typenum::Prod<
                                ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>,
                                ::typenum::consts::U8,
                            >,
                            ::typenum::Sum<
                                ::typenum::Prod<
                                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>,
                                    ::typenum::consts::U4,
                                >,
                                ::typenum::Sum<
                                    ::typenum::Prod<
                                        ::typenum::Exp<
                                            ::typenum::consts::U10,
                                            ::typenum::consts::U1,
                                        >,
                                        ::typenum::consts::U9,
                                    >,
                                    ::typenum::Sum<
                                        ::typenum::Prod<
                                            ::typenum::Exp<
                                                ::typenum::consts::U10,
                                                ::typenum::consts::U0,
                                            >,
                                            ::typenum::consts::U3,
                                        >,
                                        ::typenum::consts::U0,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >;

    typenum::assert_type_eq!(ExpectedType, ActualType);
    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <ExpectedType as typenum::ToInt<I32OrI64>>::INT,
        <ActualType as typenum::ToInt<I32OrI64>>::INT
    );
}

#[test]
fn test_nconst() {
    use core::marker::PhantomData;
    use typenum_consts::nconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = nconst![84938493];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::NInt<
        ::typenum::Sum<
            ::typenum::Prod<
                ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>,
                ::typenum::consts::U8,
            >,
            ::typenum::Sum<
                ::typenum::Prod<
                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>,
                    ::typenum::consts::U4,
                >,
                ::typenum::Sum<
                    ::typenum::Prod<
                        ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>,
                        ::typenum::consts::U9,
                    >,
                    ::typenum::Sum<
                        ::typenum::Prod<
                            ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>,
                            ::typenum::consts::U3,
                        >,
                        ::typenum::Sum<
                            ::typenum::Prod<
                                ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>,
                                ::typenum::consts::U8,
                            >,
                            ::typenum::Sum<
                                ::typenum::Prod<
                                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>,
                                    ::typenum::consts::U4,
                                >,
                                ::typenum::Sum<
                                    ::typenum::Prod<
                                        ::typenum::Exp<
                                            ::typenum::consts::U10,
                                            ::typenum::consts::U1,
                                        >,
                                        ::typenum::consts::U9,
                                    >,
                                    ::typenum::Sum<
                                        ::typenum::Prod<
                                            ::typenum::Exp<
                                                ::typenum::consts::U10,
                                                ::typenum::consts::U0,
                                            >,
                                            ::typenum::consts::U3,
                                        >,
                                        ::typenum::consts::U0,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >;

    typenum::assert_type_eq!(ExpectedType, ActualType);
    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <ExpectedType as typenum::ToInt<I32OrI64>>::INT,
        <ActualType as typenum::ToInt<I32OrI64>>::INT
    );
}

#[test]
fn test_nconst_math_exprs_no_sign() {
    use typenum::{assert_type_eq, N5};
    use typenum_consts::nconst;
    type D = nconst![{
        a = 10;
        b = 5;
        b - a; // Last statement is always the final returned value to be casted into `typenum` type-level integer, U15
    }];
    #[cfg(target_pointer_width = "32")]
    type I32OrI64 = i32;
    #[cfg(target_pointer_width = "64")]
    type I32OrI64 = i64;
    assert_eq!(
        <D as typenum::ToInt<I32OrI64>>::INT,
        <N5 as typenum::ToInt<I32OrI64>>::INT,
    );
    assert_type_eq!(D, N5);
}
