# typenum-consts

<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/jymchng/typenum-consts/ci.yml?label=build&style=for-the-badge" height="23">
  <a href="https://crates.io/crates/typenum-consts"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/typenum-consts?logo=rust&style=for-the-badge" height="23"></a>
  <a href="https://docs.rs/typenum-consts"><img alt="docs.rs" src="https://img.shields.io/crates/v/typenum-consts?color=blue&label=docs&style=for-the-badge" height="23"></a>
</div>

Procedural macros that take a literal integer (or the result of an evaluation of simple mathematical expressions or an environment variable whose value is a literal integer) and convert it to a `typenum::Unsigned` / `typenum::Integer` type-level positive/negative/unsigned integer.

There are four macros.

`tnconst![...]` can give a type-level, positive, unsigned or negative integer.

`nconst![...]` gives a type-level negative integer (e.g. `type A = nconst![69] // A is typenum::consts::N69`).

`pconst![...]` gives a type-level positive integer (e.g. `type A = pconst![69] // A is typenum::consts::P69`).

`uconst[...]` gives a type-level unsigned integer (e.g. `type A = uconst![69] // A is typenum::consts::U69`)..

## Why?

### It saves time.

Assuming you want a type-level positive integer `84938493`, `tnconst![+84938493]` outputs directly `typenum::PInt<U84938493>` (by the way, `P84938493` does not exist in `typenum::consts`). The alternative is to type `PInt<Sum<Prod<Exp<..., ...>, ...>, ...>, ...>` (which argubly takes a lot more time).

Example:

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

### For conditional compilation.

Suppose in different environments you want a different type-level integer, you can either use a configuration flag e.g. in 'production' environment, you might want the type alias `NUMBER` to be `typenum::consts::U69` and hence you might do this: `#[cfg(production)] type NUMBER = U69;`.
Alternatively, you can do the following:

```rust
use typenum::{U69, assert_type_eq};
use typenum_consts::uconst;
// ``` .env
// ENV_VAR=69
// ```
type E = uconst![env!("ENV_VAR");];
assert_type_eq!(E, U69);
```

This is when you read a literal integer directly from the environment variable and use it as a `typenum`'s type-level integer. All four macros, namely, `tnconst![...]`, `pconst![...]`, `uconst![...]` and `nconst![...]`, can read literal integers from the environment.

## Three ways to use it

Take `pconst![...]` as the example (the rest of the macros work almost identically).

### 1. Invoke it with a literal integer

```rust
use typenum::{P123, assert_type_eq};
use typenum_consts::pconst;
type A = pconst![123];
assert_type_eq!(A, P123);
```

### 2. Invoke using an expression or many simple mathematical expressions

```rust
use typenum::{P15, assert_type_eq};
use typenum_consts::pconst;
type D = pconst![{
    a = 10;
    b = 5;
    a + b; // Last statement is always the final returned value to be casted into `typenum` type-level integer, e.g. in this example a + b => 10 + 5 => 15, hence it is casted into `P15`
}];
assert_type_eq!(D, P15);
```

### 3. Invoke by reading from an environment variable

The `env!(...)` is a macro-like invocation that is used to read environment variables specifically only under the context of these four macros, namely, `tnconst!`, `pconst!`, `nconst!` and `uconst!`. The first parameter of the invocation is mandatory and it is the key of the environment variable that the invocation will read. The second parameter is optional and specifies the file path of the `.env.*` file to read the environment variable from. For example, `env!("ENV_VAR", "./.env.prod")` reads the value of `"ENV_VAR"` from `"./.env.prod"`, relative to [`CARGO_MANIFEST_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).

```rust
use typenum::{P69, assert_type_eq};
use typenum_consts::pconst;
// ``` .env
// ENV_VAR=69
// ```
type E = pconst![env!("ENV_VAR");];
assert_type_eq!(E, P69);
```

# Roadmap

- [ ] Feature gate evaluation of mathematical expressions and reading from environment variables.
- [x] Enable testing for Rust version 1.70.

## License

Licensed under either of

 * Apache License, Version 2.0

    [[LICENSE-APACHE]( http://www.apache.org/licenses/LICENSE-2.0)]
 * MIT license

    [[LICENSE-MIT](http://opensource.org/licenses/MIT)]

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
