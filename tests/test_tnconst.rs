#[test]
fn test_env_with_filepath() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![+ env!("SECRET", "tests/.env.dev");];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::PInt<
        ::typenum::Sum<
            ::typenum::Prod<
                ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>,
                ::typenum::consts::U6,
            >,
            ::typenum::Sum<
                ::typenum::Prod<
                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>,
                    ::typenum::consts::U9,
                >,
                ::typenum::Sum<
                    ::typenum::Prod<
                        ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>,
                        ::typenum::consts::U6,
                    >,
                    ::typenum::Sum<
                        ::typenum::Prod<
                            ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U0>,
                            ::typenum::consts::U9,
                        >,
                        ::typenum::consts::U0,
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
fn test_env_without_filepath() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![+ env!("ENV_VAR");];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::consts::P69;

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
fn test_env_without_semicolon() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![+ env!("ENV_VAR")];
    type _ActualType2 = tnconst![+ env!("SECRET", "tests/.env.dev")];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::P69;

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
