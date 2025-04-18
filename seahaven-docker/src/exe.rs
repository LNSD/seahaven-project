//! Executable resolution and handling
//!
//! This module provides functionality to resolve the Docker CLI executable path
//! and a type-safe wrapper for the executable path.

use std::{ffi::OsStr, path::Path};

/// Resolves and validates a Docker executable path
///
/// Takes a path-like value and attempts to locate the corresponding executable.
/// Returns an [`Executable`] if the path exists and is executable, or a [`NotFoundError`]
/// if the executable cannot be found or is not executable.
pub fn resolve<P>(path: P) -> Result<Executable, NotFoundError>
where
    P: AsRef<OsStr>,
{
    which::which(path)
        .map(|path| Executable(path.into_boxed_path()))
        .map_err(NotFoundError)
}

/// An error that can occur when resolving the Docker executable
#[derive(Debug, thiserror::Error)]
#[error("Executable not found: {0}")]
pub struct NotFoundError(which::Error);

/// The path to the Docker CLI executable
///
/// A type-safe wrapper around a [`Path`] that represents the path to the executable.
///
/// This type is used to ensure that the Docker CLI executable path is always valid.
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
