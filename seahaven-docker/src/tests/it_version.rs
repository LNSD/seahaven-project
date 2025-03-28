use crate::{
    exe::resolve_cli_executable,
    version::{get_docker_system_info_versions, get_docker_version_versions},
};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_version() {
    //* Given
    let exe = resolve_cli_executable().expect("docker binary not found");

    //* When
    let res = get_docker_version_versions(&exe).await;

    //* Then
    // Assert the different versions are greater than `0.0.0`
    let versions = res.expect("An error occurred while getting the docker version");
    assert!(versions.client > semver::Version::new(0, 0, 0));
    assert!(versions.engine > semver::Version::new(0, 0, 0));
}

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_plugin_versions() {
    //* Given
    let exe = resolve_cli_executable().expect("docker binary not found");

    //* When
    let res = get_docker_system_info_versions(&exe).await;

    //* Then
    // Assert the `compose` and `buildx` plugin versions are present and their versions are greater than `0.0.0`
    let versions = res.expect("An error occurred while getting the docker plugin versions");

    if let Some(compose_version) = versions.plugin_compose {
        assert!(compose_version > semver::Version::new(0, 0, 0));
    }
    if let Some(buildx_version) = versions.plugin_buildx {
        assert!(buildx_version > semver::Version::new(0, 0, 0));
    }
}
