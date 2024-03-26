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

// dbg!(env::var("CARGO_MANIFEST_DIR").unwrap());

// // Read the directory
// let entries = fs::read_dir(env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();

// // Iterate over the directory entries
// for entry in entries {
//     // Get the entry
//     let entry = entry.unwrap();

//     // Get the file name as a string
//     let file_name = entry.file_name();
//     let file_name = file_name.to_string_lossy();

//     // Print the file name
//     println!("{}", file_name);
// }

fn main() {
    test_tnconst();
}
