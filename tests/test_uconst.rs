#[test]
fn test_uconst_one() {
    use core::marker::PhantomData;
    use typenum_consts::uconst;

    struct Wrapper<T: typenum::Unsigned>(PhantomData<T>);

    let _wrapper = Wrapper::<uconst![84938493]>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::Sum<
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
                                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>,
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
    >;

    typenum::assert_type_eq!(ExpectedType, uconst![84938493]);
}

#[test]
fn test_tnconst_one() {
    use core::marker::PhantomData;
    use typenum_consts::tnconst;

    struct Wrapper<T: typenum::Unsigned>(PhantomData<T>);

    let _wrapper = Wrapper::<tnconst![84938493]>(PhantomData);

    #[allow(dead_code)]
    type ExpectedType = ::typenum::Sum<
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
                                    ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>,
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
    >;

    typenum::assert_type_eq!(ExpectedType, tnconst![84938493]);
}

// #[test]
// fn test_uconst_two() {
//     let _wrapper = Wrapper::<uconst![+84338433]>(PhantomData);

//     type ExpectedType = ::typenum::Sum<
//         ::typenum::Prod<
//             ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>,
//             ::typenum::consts::U8,
//         >,
//         ::typenum::Sum<
//             ::typenum::Prod<
//                 ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>,
//                 ::typenum::consts::U4,
//             >,
//             ::typenum::Sum<
//                 ::typenum::Prod<
//                     ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>,
//                     ::typenum::consts::U3,
//                 >,
//                 ::typenum::Sum<
//                     ::typenum::Prod<
//                         ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>,
//                         ::typenum::consts::U3,
//                     >,
//                     ::typenum::Sum<
//                         ::typenum::Prod<
//                             ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>,
//                             ::typenum::consts::U8,
//                         >,
//                         ::typenum::Sum<
//                             ::typenum::Prod<
//                                 ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>,
//                                 ::typenum::consts::U4,
//                             >,
//                             ::typenum::Sum<
//                                 ::typenum::Prod<
//                                     ::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>,
//                                     ::typenum::consts::U3,
//                                 >,
//                                 ::typenum::Sum<
//                                     ::typenum::Prod<
//                                         ::typenum::Exp<
//                                             ::typenum::consts::U10,
//                                             ::typenum::consts::U0,
//                                         >,
//                                         ::typenum::consts::U3,
//                                     >,
//                                     ::typenum::consts::U0,
//                                 >,
//                             >,
//                         >,
//                     >,
//                 >,
//             >,
//         >,
//     >;

//     typenum::assert_type_eq!(ExpectedType, uconst![+84338433]);
// }
