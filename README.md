# (WIP) typenum-consts

Procedural macro that takes a literal integer and converts it to a `typenum::Unsigned` type-level unsigned integer.

## Examples

```rust
use core::marker::PhantomData;
use typenum_consts::tnconst;

type ActualPositive84938493Type = tnconst![+84938493];

type ExpectedPositive84938493Type = ::typenum::PInt< // `PInt` implies positive integer at the type level
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>, ::typenum::consts::U8>, // 10**7 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>, ::typenum::consts::U4>, // 10**6 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>, ::typenum::consts::U9>, // 10**5 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>, ::typenum::consts::U3>, // 10**4 * 3
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>, ::typenum::consts::U8>, // 10**3 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>, ::typenum::consts::U4>, // 10**2 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>, ::typenum::consts::U9>, // 10**1 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U0>, ::typenum::consts::U3>, // 10**0 * 3
::typenum::consts::U0>>>>>>>>
>;

typenum::assert_type_eq!(ExpectedPositive84938493Type, ActualPositive84938493Type);
#[cfg(target_pointer_width = "32")]
type I32OrI64 = i32;
#[cfg(target_pointer_width = "64")]
type I32OrI64 = i64;
assert_eq!(
    <ExpectedPositive84938493Type as typenum::ToInt<I32OrI64>>::INT,
    <ActualPositive84938493Type as typenum::ToInt<I32OrI64>>::INT
);

type ActualNegative84938493Type = tnconst![-84938493];

type ExpectedNegative84938493Type = ::typenum::NInt< // `NInt` implies Negative integer at the type level
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>, ::typenum::consts::U8>, // 10**7 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>, ::typenum::consts::U4>, // 10**6 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>, ::typenum::consts::U9>, // 10**5 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>, ::typenum::consts::U3>, // 10**4 * 3
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>, ::typenum::consts::U8>, // 10**3 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>, ::typenum::consts::U4>, // 10**2 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>, ::typenum::consts::U9>, // 10**1 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U0>, ::typenum::consts::U3>, // 10**0 * 3
::typenum::consts::U0>>>>>>>>
>;

typenum::assert_type_eq!(ExpectedNegative84938493Type, ActualNegative84938493Type);
assert_eq!(
    <ExpectedNegative84938493Type as typenum::ToInt<I32OrI64>>::INT,
    <ActualNegative84938493Type as typenum::ToInt<I32OrI64>>::INT
);

type ActualUnsigned84938493Type = tnconst![84938493]; // No sign at the front means Unsigned

type ExpectedUnsigned84938493Type = ::typenum::Sum< // `NInt` implies Unsigned integer at the type level
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U7>, ::typenum::consts::U8>, // 10**7 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U6>, ::typenum::consts::U4>, // 10**6 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U5>, ::typenum::consts::U9>, // 10**5 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U4>, ::typenum::consts::U3>, // 10**4 * 3
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U3>, ::typenum::consts::U8>, // 10**3 * 8
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U2>, ::typenum::consts::U4>, // 10**2 * 4
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U1>, ::typenum::consts::U9>, // 10**1 * 9
::typenum::Sum<
::typenum::Prod<::typenum::Exp<::typenum::consts::U10, ::typenum::consts::U0>, ::typenum::consts::U3>, // 10**0 * 3
::typenum::consts::U0>>>>>>>>;

typenum::assert_type_eq!(ExpectedUnsigned84938493Type, ActualUnsigned84938493Type);
assert_eq!(
    <ExpectedUnsigned84938493Type as typenum::Unsigned>::USIZE,
    <ActualUnsigned84938493Type as typenum::Unsigned>::USIZE,
);
```