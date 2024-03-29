# (WIP) typenum-Procedural macro that takes a literal integer and converts it to a `typenum::Unsigned` / `typenum::ToInt` type-level positive/negative/unsigned integer.

## Examples

## Using `uconst![...]`

[`uconst!`] is a procedural macro that converts a literal integer or an expression into a [`typenum`](https://github.com/paholg/typenum/tree/main/src)'s type-level unsigned integer (i.e. the type implements the `typenum::Unsigned` trait).
There are three ways you can invoke this macro.
## 1. Invoke it with a literal integer
```rust
use typenum::{U123, assert_type_eq};
use typenum_consts::uconst;
type A = uconst![123];
assert_type_eq!(A, U123);
```
Compilation fails if the literal integer is prefixed with either a `-` or a `+`.
```compile_fail
# use typenum::{U123, assert_type_eq};
# use typenum_consts::uconst;
type B = uconst![+123]; // Fail to compile
```
```compile_fail
# use typenum::{U123, assert_type_eq};
# use typenum_consts::uconst;
type C = uconst![-123]; // Fail to compile
```
## 2. Invoke using an expression or many simple mathematical expressions
```rust
use typenum::{U15, assert_type_eq};
use typenum_consts::uconst;
type D = uconst![{
    a = 10;
    b = 5;
    a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, U15
}];
assert_type_eq!(D, U15);
```
## 3. Invoke by reading from an environment variable
Note: `env!(...)` is a macro-like invocation. The first parameter is mandatory and is the key of the environment variable that `uconst` will read. The second parameter is optional and is the file path of the `.env.*` file to read the environment variable from, e.g. `env!("ENV_VAR", "./.env.prod")`.
```rust
use typenum::{U69, assert_type_eq};
use typenum_consts::uconst;
// ``` .env
// ENV_VAR=69
// ```
type E = uconst![env!("ENV_VAR");];
assert_type_eq!(E, U69);
```

```rust
# use core::marker::PhantomData;
# use typenum_consts::tnconst;
# use typenum::*;
#
# #[cfg(target_pointer_width = "32")]
# type I32OrI64 = i32;
# #[cfg(target_pointer_width = "64")]
# type I32OrI64 = i64;

type ActualPositive84938493Type = tnconst![+84938493];
type ExpectedPositive84938493Type = PInt< // `PInt` implies positive integer at the type level
Sum<
Prod<Exp<U10, U7>, U8>, // 10**7 * 8
Sum<
Prod<Exp<U10, U6>, U4>, // 10**6 * 4
Sum<
Prod<Exp<U10, U5>, U9>, // 10**5 * 9
Sum<
Prod<Exp<U10, U4>, U3>, // 10**4 * 3
Sum<
Prod<Exp<U10, U3>, U8>, // 10**3 * 8
Sum<
Prod<Exp<U10, U2>, U4>, // 10**2 * 4
Sum<
Prod<Exp<U10, U1>, U9>, // 10**1 * 9
Sum<
Prod<Exp<U10, U0>, U3>, // 10**0 * 3
U0>>>>>>>>
>;

typenum::assert_type_eq!(ExpectedPositive84938493Type, ActualPositive84938493Type);
assert_eq!(
    <ExpectedPositive84938493Type as typenum::ToInt<I32OrI64>>::INT,
    <ActualPositive84938493Type as typenum::ToInt<I32OrI64>>::INT
);
```
```rust
# use core::marker::PhantomData;
# use typenum_consts::tnconst;
# use typenum::*;
#
# #[cfg(target_pointer_width = "32")]
# type I32OrI64 = i32;
# #[cfg(target_pointer_width = "64")]
# type I32OrI64 = i64;

type ActualNegative84938493Type = tnconst![-84938493];

type ExpectedNegative84938493Type = NInt< // `NInt` implies Negative integer at the type level
Sum<
Prod<Exp<U10, U7>, U8>, // 10**7 * 8
Sum<
Prod<Exp<U10, U6>, U4>, // 10**6 * 4
Sum<
Prod<Exp<U10, U5>, U9>, // 10**5 * 9
Sum<
Prod<Exp<U10, U4>, U3>, // 10**4 * 3
Sum<
Prod<Exp<U10, U3>, U8>, // 10**3 * 8
Sum<
Prod<Exp<U10, U2>, U4>, // 10**2 * 4
Sum<
Prod<Exp<U10, U1>, U9>, // 10**1 * 9
Sum<
Prod<Exp<U10, U0>, U3>, // 10**0 * 3
U0>>>>>>>>
>;

typenum::assert_type_eq!(ExpectedNegative84938493Type, ActualNegative84938493Type);
assert_eq!(
    <ExpectedNegative84938493Type as typenum::ToInt<I32OrI64>>::INT,
    <ActualNegative84938493Type as typenum::ToInt<I32OrI64>>::INT
);
```
```rust
# use core::marker::PhantomData;
# use typenum_consts::tnconst;
# use typenum::*;
#
# #[cfg(target_pointer_width = "32")]
# type I32OrI64 = i32;
# #[cfg(target_pointer_width = "64")]
# type I32OrI64 = i64;

type ActualUnsigned84938493Type = tnconst![84938493]; // No sign at the front means Unsigned

type ExpectedUnsigned84938493Type = Sum< // No `PInt` or `NInt` implies Unsigned integer at the type level
Prod<Exp<U10, U7>, U8>, // 10**7 * 8
Sum<
Prod<Exp<U10, U6>, U4>, // 10**6 * 4
Sum<
Prod<Exp<U10, U5>, U9>, // 10**5 * 9
Sum<
Prod<Exp<U10, U4>, U3>, // 10**4 * 3
Sum<
Prod<Exp<U10, U3>, U8>, // 10**3 * 8
Sum<
Prod<Exp<U10, U2>, U4>, // 10**2 * 4
Sum<
Prod<Exp<U10, U1>, U9>, // 10**1 * 9
Sum<
Prod<Exp<U10, U0>, U3>, // 10**0 * 3
U0>>>>>>>>;

typenum::assert_type_eq!(ExpectedUnsigned84938493Type, ActualUnsigned84938493Type);
assert_eq!(
    <ExpectedUnsigned84938493Type as typenum::Unsigned>::USIZE,
    <ActualUnsigned84938493Type as typenum::Unsigned>::USIZE,
);
```

# Vendored Crates

## [`rsc`](https://github.com/fivemoreminix/rsc/commit/67c4ddffbe45a30de0fd696c569de885bfd4e9b4) version 3.0.0

Reasons for vendoring `src`.

1. As of 28 March 2024, there is a version 3.0.0 of the crate on the GitHub [repository](https://github.com/fivemoreminix/rsc/commit/67c4ddffbe45a30de0fd696c569de885bfd4e9b4) but without corresponding crate on `crates.io`.
2. Easier to implement `Num` for `isize` with vendoring.
3. `typenum-needs a mathematical expression evaluator.
4. Its [license](https://github.com/fivemoreminix/rsc/tree/67c4ddffbe45a30de0fd696c569de885bfd4e9b4?tab=readme-ov-file#license) allows for 'usage without attribution'. Anyway, `src/vendors/rsc/Cargo.toml.vendored` is the original `Cargo.toml` file found in the repository.
5. Thanks to [Luke Wilson](https://github.com/fivemoreminix).
