use std::{ffi::OsStr, path::Path};

/// The environment variable that contains the path to the docker CLI binary
const DOCKER_CLI_PATH_ENVVAR: &str = "SEAHAVEN_DOCKER_CLI";

/// Get the docker CLI binary path from the environment
///
/// If the [`DOCKER_CLI_PATH_ENVVAR`] environment variable is set, it will be used.
/// Otherwise, the function will try to find the `docker` CLI binary in the "$PATH" environment variable.
/// If the `docker` CLI binary is not found, an error is returned.
///
/// Returns the path to the docker CLI binary.
pub fn resolve_docker_cli_binary() -> anyhow::Result<CliBinary> {
    let path = std::env::var_os(DOCKER_CLI_PATH_ENVVAR).unwrap_or_else(|| "docker".into());
    which::which(path)
        .map(|path| CliBinary(path.into_boxed_path()))
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker CLI binary: {}", err))
}

/// The docker CLI binary
///
/// A *new-type* wrapper around a [`Path`] that represents the path to the docker CLI binary.
///
/// This type is used to ensure that the docker CLI binary path is always valid.
pub struct CliBinary(Box<Path>);

impl AsRef<Path> for CliBinary {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<OsStr> for CliBinary {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}
