use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();
    t.compile_fail("trybuild_tests/compile_fails/*.rs");
    t.pass("trybuild_tests/compile_passes/*.rs");
}
