#[test]
fn test_file_metadata() {
    assert_wasi_output!(
        "../../wasitests/file_metadata.wasm",
        "file_metadata",
        vec![],
        vec![],
        "../../wasitests/file_metadata.out"
    );
}
