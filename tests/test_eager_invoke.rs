#[test]
fn test_eager_invoke() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::NonZero>(PhantomData<T>);

    // type ActualType = tnconst![
    //     #[cfg(feature = "debug")]
    //     69
    //     #[cfg(not(feature = "debug"))]
    //     96
    // ];
    type ActualType = if core::cfg!(feature = "debug") { typenum::consts::U69 } else { typenum::consts::U96 };

    #[cfg(feature = "debug")]
    type ExpectedType = typenum::consts::U69;

    #[cfg(not(feature = "debug"))]
    type ExpectedType = typenum::consts::U96;

    let _wrapper = Wrapper::<ActualType>(PhantomData);

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
