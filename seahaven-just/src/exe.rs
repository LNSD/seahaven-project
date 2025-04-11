//! Executable resolution and handling
//!
//! This module provides functionality to resolve the Just CLI executable path
//! and a type-safe wrapper for the executable path.
//!
//! ## Executable resolution
//!
//! The Just CLI executable is resolved in the following order:
//!
//!  1. If the `SEAHAVEN_JUST_CLI` environment variable is set, its value is used as the
//!     executable path.
//!  2. Otherwise, the system's `just` command is located in the `$PATH` environment variable.
//!
//! In both cases, the resolved path is validated to ensure it exists and is executable.
//! If the path is invalid or the executable cannot be found, an error is returned.

use std::{ffi::OsStr, path::Path};

/// The environment variable that contains the path to the just CLI binary
const JUST_CLI_EXECUTABLE_ENVVAR: &str = "SEAHAVEN_JUST_CLI";

/// The default just CLI executable name
const DEFAULT_JUST_CLI_EXECUTABLE: &str = "just";

/// Errors that can occur when resolving the Just CLI executable
#[derive(Debug, thiserror::Error)]
#[error("Just CLI executable not found: {0}")]
pub struct NotFoundError(#[from] which::Error);

/// Resolve the just CLI executable path
///
/// If the [`JUST_CLI_PATH_ENVVAR`] environment variable is set, it will be used.
/// Otherwise, the function will try to find the `just` CLI executable in the "$PATH" environment variable.
/// If the `just` CLI executable is not found, an error is returned.
///
/// Returns the path to the just CLI executable.
pub fn resolve_cli_executable() -> Result<Executable, NotFoundError> {
    let path = std::env::var_os(JUST_CLI_EXECUTABLE_ENVVAR)
        .unwrap_or_else(|| DEFAULT_JUST_CLI_EXECUTABLE.into());
    which::which(path)
        .map(|path| Executable(path.into_boxed_path()))
        .map_err(Into::into)
}

/// The path to the Just CLI executable
///
/// A type-safe wrapper around a [`Path`] that represents the path to the executable.
///
/// This type is used to ensure that the Just CLI executable path is always valid.
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
