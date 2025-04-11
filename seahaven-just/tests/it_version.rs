use seahaven_just::{exe::resolve_cli_executable, version};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_just_version() {
    //* Given
    let exe = resolve_cli_executable().expect("just binary not found");

    //* When
    let res = version::fetch(&exe).await;

    //* Then
    let version = res.expect("An error occurred while getting the just version");

    // Assert the version is greater than `1.0.0`
    assert!(version > semver::Version::new(1, 0, 0));
}
