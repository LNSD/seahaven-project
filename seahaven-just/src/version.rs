//! Version information retrieval for just command
//!
//! This module provides functionality to retrieve version details from the Just CLI.
//!
//! This version information is essential for ensuring compatibility and for troubleshooting issues
//! when executing just commands.

use std::borrow::Borrow;

use semver::Version;

use crate::{
    cmd::{IntoCommand, JustCmd},
    exe::Executable,
};

/// Errors that can occur when retrieving Just version information
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to execute the Just command
    #[error("Failed to execute '{cmd}': {src}")]
    CommandExecution {
        cmd: String,
        #[source]
        src: std::io::Error,
    },

    /// Failed to parse the Just command output as UTF-8
    #[error("Failed to parse '{cmd}' output: {src}")]
    OutputUtf8Parsing {
        cmd: String,
        #[source]
        src: std::string::FromUtf8Error,
    },

    /// Failed to parse the Just command format
    #[error("Failed to parse '{cmd}' output: {src}")]
    OutputFormatParsing {
        cmd: String,
        #[source]
        src: std::io::Error,
    },

    /// Failed to parse a version string
    #[error("Failed to parse '{cmd}' version string: {src}")]
    VersionParsing {
        cmd: String,
        #[source]
        src: semver::Error,
    },
}

/// Fetch the Just version
///
/// This function retrieves the version of the Just CLI.
///
/// This information is used to ensure compatibility when executing `just` commands.
///
/// # Errors
///
/// This function will return an error if the `just --version` command fails to execute, or if the output cannot be parsed.
pub async fn fetch<E>(bin: &E) -> Result<Version, Error>
where
    E: Borrow<Executable>,
{
    // Create the command
    let mut cmd = JustCmd::with_executable(bin.borrow())
        .version()
        .into_command();
    let cmd_str = format!("{:?}", cmd.as_std());

    // Execute the command
    let output = cmd
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err,
        })?;

    // Parse the command stdout as UTF-8 string
    let output_str = String::from_utf8(output.stdout).map_err(|err| Error::OutputUtf8Parsing {
        cmd: cmd_str.clone(),
        src: err,
    })?;

    // Extract the version string from the output. Expected format: `just X.YY.ZZ`
    let version_str =
        output_str
            .trim()
            .strip_prefix("just ")
            .ok_or_else(|| Error::OutputFormatParsing {
                cmd: cmd_str.clone(),
                src: std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid version format: {output_str}"),
                ),
            })?;

    // Parse the version string
    let version = semver::Version::parse(version_str).map_err(|err| Error::VersionParsing {
        cmd: cmd_str.clone(),
        src: err,
    })?;

    Ok(version)
}
