# Cargo test only 1 test

cargo test --features debug --test test_eager_invoke -- test_eager_invoke --exact --nocapture

# Parsing Vec<Attribute> and Values

https://stackoverflow.com/a/76089971

# Eager proc-macro Expansion

https://stackoverflow.com/questions/77153497/eager-proc-macro-expansion/77160978#77160978

# Run tests with debug prints

export ENV_VAR=69 && cargo test --config 'build.rustflags=["--cfg", "__debug_tnconst"]'

# Run tests without debug prints

export ENV_VAR=69 && cargo test

# Vendoring doesn't work

1. Vendor specific crate from GitHub doesn't work - https://github.com/rust-lang/cargo/issues/9234
2. Git submodule add and checkout-ed to specific commit works
3. but, even after replacing Cargo.toml's [source.<link-to-git-repo>], `cargo package` still attempts to find the dep on `crates.io`

https://users.rust-lang.org/t/publishing-a-package-with-a-vendored-crate-but-is-not-listed-as-a-dependency-because-the-vendored-crate-author-did-not-publish-his-crate-onto-crates-io-lol/108971/1
