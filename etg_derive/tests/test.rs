#[test]
fn test_tags() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/01_happy_path.rs");
    t.compile_fail("tests/cases/02_fail_for_struct.rs");
    t.compile_fail("tests/cases/03_mandatory_serde_rename.rs");
    t.compile_fail("tests/cases/04_fail_on_tuple_variant.rs");
    t.compile_fail("tests/cases/05_fail_on_struct_variant.rs");
}
