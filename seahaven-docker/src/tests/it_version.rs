use crate::{
    exe::resolve,
    version::{get_docker_system_info_versions, get_docker_version_versions},
};

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_version() {
    //* Given
    let exe = resolve("docker").expect("docker binary not found");

    //* When
    let res = get_docker_version_versions(&exe).await;

    //* Then
    let versions = res.expect("An error occurred while getting the docker version");

    // Assert the client version is greater than `0.0.0`
    assert!(versions.client > semver::Version::new(0, 0, 0));

    // Assert the engine version, if present, is greater than `0.0.0`
    if let Some(engine_version) = versions.engine {
        assert!(engine_version > semver::Version::new(0, 0, 0));
    }
}

#[test_with::no_env(CI)]
#[tokio::test]
async fn resolve_docker_plugin_versions() {
    //* Given
    let exe = resolve("docker").expect("docker binary not found");

    //* When
    let res = get_docker_system_info_versions(&exe).await;

    //* Then
    let versions = res.expect("An error occurred while getting the docker plugin versions");

    // Assert the `compose` plugin version, if present, is greater than `0.0.0`
    if let Some(compose_version) = versions.plugin_compose {
        assert!(compose_version > semver::Version::new(0, 0, 0));
    }

    // Assert the `buildx` plugin version, if present, is greater than `0.0.0`
    if let Some(buildx_version) = versions.plugin_buildx {
        assert!(buildx_version > semver::Version::new(0, 0, 0));
    }
}
