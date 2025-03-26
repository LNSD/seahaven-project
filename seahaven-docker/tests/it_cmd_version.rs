use seahaven_docker::cmd::{
    bin::resolve_docker_cli_binary,
    version::{get_docker_plugin_versions, get_docker_version},
};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_version() {
    //* Given
    let bin = resolve_docker_cli_binary().expect("docker binary not found");

    //* When
    let res = get_docker_version(&bin).await;

    //* Then
    // Assert the different versions are greater than `0.0.0`
    let docker_version = res.expect("docker version not found");
    assert!(docker_version.client > semver::Version::new(0, 0, 0));
    assert!(docker_version.engine > semver::Version::new(0, 0, 0));
}

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_plugin_versions() {
    //* Given
    let bin = resolve_docker_cli_binary().expect("docker binary not found");

    //* When
    let res = get_docker_plugin_versions(&bin).await;

    //* Then
    // Assert the `compose` and `buildx` plugin versions are present and their versions are greater than `0.0.0`
    let plugin_versions = res.expect("docker plugin versions not found");

    assert!(plugin_versions.compose.is_some());
    let compose_version = plugin_versions
        .compose
        .expect("docker compose plugin not found");
    assert!(compose_version > semver::Version::new(0, 0, 0));

    assert!(plugin_versions.buildx.is_some());
    let buildx_version = plugin_versions
        .buildx
        .expect("docker buildx plugin not found");
    assert!(buildx_version > semver::Version::new(0, 0, 0));
}
