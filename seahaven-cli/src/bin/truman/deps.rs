//! # Truman CLI Dependencies
//!
//! This module provides functionality for resolving and managing external dependencies
//! required by the Truman CLI, specifically Docker and Just executables.
//!
//! It handles:
//! - Resolution of executable paths through environment variables or system PATH
//! - Version information retrieval for both Docker and Just
//! - Error handling for missing or inaccessible executables

use seahaven_docker::exe::{Executable as DockerExecutable, NotFoundError as DockerNotFoundError};
use seahaven_just::exe::{Executable as JustExecutable, NotFoundError as JustNotFoundError};

/// Environment variable name for specifying a custom Docker executable path
const DOCKER_EXE_ENV_VAR: &str = "SEAHAVEN_DOCKER_EXE";

/// Default name of the Docker executable when not specified via environment variable
const DEFAULT_DOCKER_EXE: &str = "docker";

/// Environment variable name for specifying a custom Just executable path
const JUST_EXE_ENV_VAR: &str = "SEAHAVEN_JUST_EXE";

/// Default name of the Just executable when not specified via environment variable
const DEFAULT_JUST_EXE: &str = "just";

/// Resolves the path to the Docker CLI executable.
///
/// The resolution process follows this order:
/// 1. Checks for the [`DOCKER_EXE_ENV_VAR`] environment variable
/// 2. Falls back to the default executable name if no environment variable is set
/// 3. Searches for the executable in the system PATH
///
/// # Returns
///
/// - `Ok(DockerExecutable)` containing the resolved executable path
/// - `Err(DockerNotFoundError)` if the executable cannot be found or accessed
///
/// # Errors
///
/// Returns a [`DockerNotFoundError`] if:
/// - The specified path in the environment variable does not exist
/// - The default executable cannot be found in the system PATH
/// - The executable exists but is not accessible
pub fn resolve_docker_executable() -> Result<DockerExecutable, DockerNotFoundError> {
    let path = std::env::var_os(DOCKER_EXE_ENV_VAR).unwrap_or(DEFAULT_DOCKER_EXE.into());
    seahaven_docker::exe::resolve(path)
}

/// Resolves the path to the Just CLI executable.
///
/// The resolution process follows this order:
/// 1. Checks for the [`JUST_EXE_ENV_VAR`] environment variable
/// 2. Falls back to the default executable name if no environment variable is set
/// 3. Searches for the executable in the system PATH
///
/// # Returns
///
/// - `Ok(JustExecutable)` containing the resolved executable path
/// - `Err(JustNotFoundError)` if the executable cannot be found or accessed
///
/// # Errors
///
/// Returns a [`JustNotFoundError`] if:
/// - The specified path in the environment variable does not exist
/// - The default executable cannot be found in the system PATH
/// - The executable exists but is not accessible
pub fn resolve_just_executable() -> Result<JustExecutable, JustNotFoundError> {
    let path = std::env::var_os(JUST_EXE_ENV_VAR).unwrap_or(DEFAULT_JUST_EXE.into());
    seahaven_just::exe::resolve(path)
}

/// Retrieves version information for the Docker installation.
///
/// This function:
/// 1. Resolves the Docker executable path
/// 2. Executes the Docker CLI to fetch version information
/// 3. Parses and returns the version details
///
/// # Returns
///
/// - `Some(Version)` containing the Docker version information if successful
/// - `None` if:
///   - The Docker executable cannot be found
///   - Version information cannot be retrieved
///   - The version check fails
///
/// Failures are logged at the debug level and do not propagate errors.
pub async fn fetch_docker_version() -> Option<seahaven_docker::version::Version> {
    let docker_exe = match resolve_docker_executable() {
        Ok(exe) => exe,
        Err(err) => {
            tracing::debug!("Failed to resolve docker executable: {}", err);
            return None;
        }
    };

    match seahaven_docker::version::fetch(&docker_exe).await {
        Ok(version) => Some(version),
        Err(err) => {
            tracing::debug!("Failed to determine docker version: {}", err);
            None
        }
    }
}

/// Retrieves version information for the Just installation.
///
/// This function:
/// 1. Resolves the Just executable path
/// 2. Executes the Just CLI to fetch version information
/// 3. Parses and returns the version details
///
/// # Returns
///
/// - `Some(Version)` containing the Just version information if successful
/// - `None` if:
///   - The Just executable cannot be found
///   - Version information cannot be retrieved
///   - The version check fails
///
/// Failures are logged at the debug level and do not propagate errors.
pub async fn fetch_just_version() -> Option<seahaven_just::version::Version> {
    let just_exe = match resolve_just_executable() {
        Ok(exe) => exe,
        Err(err) => {
            tracing::debug!("Failed to resolve just executable: {}", err);
            return None;
        }
    };

    match seahaven_just::version::fetch(&just_exe).await {
        Ok(version) => Some(version),
        Err(err) => {
            tracing::debug!("Failed to determine just version: {}", err);
            None
        }
    }
}
