# Cargo test only 1 test

cargo test --features debug --test test_eager_invoke -- test_eager_invoke --exact --nocapture

# Parsing Vec<Attribute> and Values

https://stackoverflow.com/a/76089971

# Eager proc-macro Expansion

https://stackoverflow.com/questions/77153497/eager-proc-macro-expansion/77160978#77160978

# Run tests with debug prints

export ENV_VAR=69 && cargo test --config 'build.rustflags=["--cfg", "__debug_tnconst"]'
