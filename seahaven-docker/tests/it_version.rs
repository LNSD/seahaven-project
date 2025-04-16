use seahaven_docker::{exe::resolve_cli_executable, version};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_version() {
    //* Given
    let exe = resolve_cli_executable().expect("docker binary not found");

    //* When
    let res = version::fetch(&exe).await;

    //* Then
    let version = res.expect("An error occurred while getting the docker version");

    // Assert the CLI version is greater than `0.0.0`
    assert!(version.cli > semver::Version::new(0, 0, 0));

    // Assert the client and engine versions are greater than `0.0.0`
    if let Some(client_version) = version.client {
        assert!(client_version > semver::Version::new(0, 0, 0));
    }
    if let Some(engine_version) = version.engine {
        assert!(engine_version > semver::Version::new(0, 0, 0));
    }

    // Assert the compose plugin version, if present, is greater than `0.0.0`
    if let Some(version) = version.plugin_compose {
        assert!(version > semver::Version::new(0, 0, 0));
    }

    // Assert the buildx plugin version, if present, is greater than `0.0.0`
    if let Some(version) = version.plugin_buildx {
        assert!(version > semver::Version::new(0, 0, 0));
    }
}
