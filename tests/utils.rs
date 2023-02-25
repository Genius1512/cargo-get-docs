use cargo_get_docs::utils::does_crate_exist;

#[test]
fn does_crate_exist_test() {
    assert!(does_crate_exist("anyhow").unwrap());
    assert!(!does_crate_exist("sdfg").unwrap());
}
