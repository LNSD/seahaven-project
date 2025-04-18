use seahaven_just::exe::resolve;

#[test_with::no_env(CI)]
#[test]
fn resolve_invalid_executable() {
    //* Given
    let invalid_exe_name = "non_existent_just_binary";

    //* When
    let invalid_result = resolve(invalid_exe_name);

    //* Then
    assert!(
        invalid_result.is_err(),
        "Should fail with non-existent binary"
    );
}

#[test_with::no_env(CI)]
#[test]
fn executable_display_and_debug() {
    //* Given
    let exe = resolve("just").expect("just binary not found");

    //* When
    let display_str = format!("{}", exe);
    let debug_str = format!("{:?}", exe);

    //* Then
    assert!(display_str.contains("just"));
    assert!(debug_str.contains("just"));
}
