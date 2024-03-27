/// export ENV_VAR="69" && cargo test

fn test_tnconst() {
    // use std::{env, fs};
    use typenum::{assert_type_eq, consts::*};
    use typenum_consts::tnconst;

    type A = tnconst![+ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::PInt
    type B = tnconst![- env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::NInt
    type C = tnconst![ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::UInt

    assert_type_eq!(A, P69);
    assert_type_eq!(B, N69);
    assert_type_eq!(C, U69);

    type D = tnconst![+ 123];
    type E = tnconst![-123];
    type F = tnconst![123];

    assert_type_eq!(D, P123);
    assert_type_eq!(E, N123);
    assert_type_eq!(F, U123);

    type H = tnconst![+ {
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, P69
    }];
    type I = tnconst![-{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, N69
    }];
    type J = tnconst![{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, U69
    }];

    assert_type_eq!(H, P69);
    assert_type_eq!(I, N69);
    assert_type_eq!(J, U69);

    println!("`test_tnconst` Passed!");
}

fn test_pconst() {
    // use std::{env, fs};
    use typenum::{assert_type_eq, consts::*};
    use typenum_consts::pconst;

    type A = pconst![+ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::PInt
    type C = pconst![ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::UInt

    assert_type_eq!(A, P69);
    assert_type_eq!(C, P69);

    type D = pconst![+ 123];
    type F = pconst![123];

    assert_type_eq!(D, P123);
    assert_type_eq!(F, P123);

    type H = pconst![+ {
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, P69
    }];
    type J = pconst![{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, U69
    }];

    assert_type_eq!(H, P69);
    assert_type_eq!(J, P69);

    println!("`test_pconst` Passed!");
}

fn test_nconst() {
    // use std::{env, fs};
    use typenum::{assert_type_eq, consts::*};
    use typenum_consts::nconst;

    type B = nconst![- env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::NInt
    type C = nconst![ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::UInt

    assert_type_eq!(B, N69);
    assert_type_eq!(C, N69);

    type E = nconst![-123];
    type F = nconst![123];

    assert_type_eq!(E, N123);
    assert_type_eq!(F, N123);

    type I = nconst![-{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, N69
    }];
    type J = nconst![{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, U69
    }];

    assert_type_eq!(I, N69);
    assert_type_eq!(J, N69);

    println!("`test_nconst` Passed!");
}

fn test_uconst() {
    // use std::{env, fs};
    use typenum::{assert_type_eq, consts::*};
    use typenum_consts::uconst;

    type C = uconst![ env!("ENV_VAR");]; // Read from environment, get a literal int => typenum::UInt

    assert_type_eq!(C, U69);

    type F = uconst![123];

    assert_type_eq!(F, U123);

    type J = uconst![{
        a = 10;
        b = 5;
        a + b;
        c = 69 // Last statement is always the final returned value to be casted into `typenum` type-level integer, U69
    }];

    assert_type_eq!(J, U69);

    println!("`test_uconst` Passed!");
}

fn main() {
    test_tnconst();
    test_pconst();
    test_nconst();
    test_uconst();
}
