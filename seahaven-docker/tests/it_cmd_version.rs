use seahaven_docker::{
    exe::resolve_cli_executable,
    version::{get_docker_plugin_versions, get_docker_version},
};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_version() {
    //* Given
    let bin = resolve_cli_executable().expect("docker binary not found");

    //* When
    let res = get_docker_version(&bin).await;

    //* Then
    // Assert the different versions are greater than `0.0.0`
    let docker_version = res.expect("An error occurred while getting the docker version");
    assert!(docker_version.client > semver::Version::new(0, 0, 0));
    assert!(docker_version.engine > semver::Version::new(0, 0, 0));
}

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_plugin_versions() {
    //* Given
    let bin = resolve_cli_executable().expect("docker binary not found");

    //* When
    let res = get_docker_plugin_versions(&bin).await;

    //* Then
    // Assert the `compose` and `buildx` plugin versions are present and their versions are greater than `0.0.0`
    let plugin_versions = res.expect("An error occurred while getting the docker plugin versions");

    if let Some(compose_version) = plugin_versions.compose {
        assert!(compose_version > semver::Version::new(0, 0, 0));
    }
    if let Some(buildx_version) = plugin_versions.buildx {
        assert!(buildx_version > semver::Version::new(0, 0, 0));
    }
}
