#[test]
fn test_exprs() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    type ActualType = tnconst![-{
        x = 69;
        x = x + 1;
        x = x - 1;
        x;
    }];

    let _wrapper = Wrapper::<ActualType>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = typenum::consts::N69;

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
