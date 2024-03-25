fn main() {
    use std::{env, fs};
    use typenum_consts::tnconst;

    dbg!(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Read the directory
    let entries = fs::read_dir(env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();

    // Iterate over the directory entries
    for entry in entries {
        // Get the entry
        let entry = entry.unwrap();

        // Get the file name as a string
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        // Print the file name
        println!("{}", file_name);
    }

    type A = tnconst![+ env!("SEXY_SECRET");];
    type D = tnconst![+ env!("SEXY_SECRET")];
    type F = tnconst![+ env!("SECRET", "../../../../../tests/.env.dev");];
    type B = tnconst![+ {
        a = 10;
        b = 5;
        a + b;
        c = 69
    }];
    type C = tnconst![+ 123456];
}
