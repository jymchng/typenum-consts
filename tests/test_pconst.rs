#[test]
fn test_tnconst_three() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![+84938493];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::PInt<
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
fn test_pconst() {
    use core::marker::PhantomData;
    use typenum_consts::pconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = pconst![+84938493];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::PInt<
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
fn test_file_path_works() {
    use typenum::{assert_type_eq, consts::P69};
    use typenum_consts::pconst;

    assert_type_eq!(pconst![+ env!("ENV_VAR", "tests/.env.dev");], P69);
    assert_type_eq!(pconst![env!("ENV_VAR", "tests/.env.dev")], P69);
}
