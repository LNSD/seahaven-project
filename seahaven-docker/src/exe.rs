use std::{ffi::OsStr, path::Path};

/// The environment variable that contains the path to the docker CLI binary
const DOCKER_CLI_PATH_ENVVAR: &str = "SEAHAVEN_DOCKER_CLI";

/// Resolve the docker CLI executable path
///
/// If the [`DOCKER_CLI_PATH_ENVVAR`] environment variable is set, it will be used.
/// Otherwise, the function will try to find the `docker` CLI executable in the "$PATH" environment variable.
/// If the `docker` CLI executable is not found, an error is returned.
///
/// Returns the path to the docker CLI executable.
pub fn resolve_cli_executable() -> anyhow::Result<Executable> {
    let path = std::env::var_os(DOCKER_CLI_PATH_ENVVAR).unwrap_or_else(|| "docker".into());
    which::which(path)
        .map(|path| Executable(path.into_boxed_path()))
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker CLI executable: {}", err))
}

/// The path to the docker CLI executable
///
/// A *new-type* wrapper around a [`Path`] that represents the path to the docker CLI executable.
///
/// This type is used to ensure that the docker CLI executable path is always valid.
#[derive(Clone)]
pub struct Executable(Box<Path>);

impl AsRef<OsStr> for Executable {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl std::fmt::Display for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl std::fmt::Debug for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
