//! Executable resolution and handling
//!
//! This module provides functionality to resolve the Docker CLI executable path
//! and a type-safe wrapper for the executable path.
//!
//! ## Executable resolution
//!
//! The Docker CLI executable is resolved in the following order:
//!
//!  1. If the `SEAHAVEN_DOCKER_CLI` environment variable is set, its value is used as the
//!     executable path.
//!  2. Otherwise, the system's `docker` command is located in the `$PATH` environment variable.
//!
//! In both cases, the resolved path is validated to ensure it exists and is executable.
//! If the path is invalid or the executable cannot be found, an error is returned.
//!
//! ## Example
//!
//! Resolving the docker CLI executable from the `$PATH`:
//!
//! ```rust
//! use seahaven_docker::exe::Executable;
//!
//! let exe = Executable::resolve("docker").expect("Failed to resolve docker CLI executable");
//! ```
use std::{ffi::OsStr, path::Path};

/// The environment variable that contains the path to the docker CLI binary
const DOCKER_CLI_EXECUTABLE_ENVVAR: &str = "SEAHAVEN_DOCKER_CLI";

/// The default docker CLI executable name
const DEFAULT_DOCKER_CLI_EXECUTABLE: &str = "docker";

/// Errors that can occur when resolving the Docker CLI executable
#[derive(Debug, thiserror::Error)]
#[error("Docker CLI executable not found: {0}")]
pub struct NotFoundError(#[from] which::Error);

/// Resolve the docker CLI executable path
///
/// If the [`DOCKER_CLI_PATH_ENVVAR`] environment variable is set, it will be used.
/// Otherwise, the function will try to find the `docker` CLI executable in the "$PATH" environment variable.
/// If the `docker` CLI executable is not found, an error is returned.
///
/// Returns the path to the docker CLI executable.
pub fn resolve_cli_executable() -> Result<Executable, NotFoundError> {
    let path = std::env::var_os(DOCKER_CLI_EXECUTABLE_ENVVAR)
        .unwrap_or_else(|| DEFAULT_DOCKER_CLI_EXECUTABLE.into());
    which::which(path)
        .map(|path| Executable(path.into_boxed_path()))
        .map_err(Into::into)
}

/// The path to the Docker CLI executable
///
/// A type-safe wrapper around a [`Path`] that represents the path to the executable.
///
/// This type is used to ensure that the Docker CLI executable path is always valid.
#[derive(Clone)]
pub struct Executable(Box<Path>);

impl Executable {
    /// Create a new [`Executable`] from a path
    ///
    /// The path is resolved and validated to ensure it exists and is executable.
    /// If the path is not found, or is not executable, an error is returned.
    pub fn resolve<P>(path: P) -> Result<Self, NotFoundError>
    where
        P: AsRef<OsStr>,
    {
        which::which(path)
            .map(|path| Executable(path.into_boxed_path()))
            .map_err(Into::into)
    }
}

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
