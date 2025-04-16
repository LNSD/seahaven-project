//! Version information retrieval
//!
//! This module provides functionality to retrieve version details from different Docker components:
//! - The Docker client version (CLI)
//! - The Docker engine version (server)
//! - Docker plugins versions
//!
//! This version information is essential for ensuring compatibility and for troubleshooting issues.

use std::{borrow::Borrow, process::Stdio, time::Duration};

pub use semver;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{DockerCmd, IntoCommand},
    exe::Executable,
};

/// The Docker version
///
/// This struct contains the version information of the different Docker components.
///
/// See [`fetch`] for more information.
#[derive(Debug, Clone)]
pub struct Version {
    /// The CLI version
    ///
    /// The version reported by the `docker --version` command.
    pub cli: semver::Version,

    /// The client version
    ///
    /// The version reported by the `docker version` command.
    ///
    /// The `docker version` can fail if the Docker daemon is not running. In this case, the version will be `None`.
    pub client: Option<semver::Version>,

    /// The engine version
    ///
    /// The version reported by the `docker version` command.
    ///
    /// The `docker version` can fail if the Docker daemon is not running. In this case, the version will be `None`.
    pub engine: Option<semver::Version>,

    /// The buildx plugin version
    ///
    /// The version reported by the `docker system info` command.
    ///
    /// If the plugin is not installed, this will be `None`.
    pub plugin_buildx: Option<semver::Version>,

    /// The compose plugin version
    ///
    /// The version reported by the `docker system info` command.
    ///
    /// If the plugin is not installed, this will be `None`.
    pub plugin_compose: Option<semver::Version>,
}

/// Errors that can occur when retrieving Docker version information
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to execute the Docker command
    #[error("Failed to execute '{cmd}': {src}")]
    CommandExecution {
        cmd: String,
        #[source]
        src: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to parse the Docker version output
    #[error("Failed to parse '{cmd}' output: {src}")]
    OutputParsing {
        cmd: String,
        #[source]
        src: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to parse a version string
    #[error("Failed to parse '{cmd}' version string: {src}")]
    VersionParsing {
        cmd: String,
        #[source]
        src: semver::Error,
    },
}

/// Fetch the Docker version
///
/// This function retrieves the version of the Docker client and engine, as well as the versions of the Docker compose and buildx plugins.
///
/// # Errors
///
/// This function will return an error if the `docker version` command fails to execute, or if the output cannot be parsed.
pub async fn fetch<E>(bin: &E) -> Result<Version, Error>
where
    E: Borrow<Executable>,
{
    let (cli_version, version, system_info) = tokio::join!(
        get_docker_cli_version(bin.borrow()),
        get_docker_version_versions(bin.borrow()),
        get_docker_system_info_versions(bin.borrow())
    );

    let (client_version, engine_version) = match version {
        Ok(version) => (Some(version.client), version.engine),
        Err(_) => (None, None),
    };

    let (plugin_compose, plugin_buildx) = match system_info {
        Ok(system_info) => (system_info.plugin_compose, system_info.plugin_buildx),
        Err(_) => (None, None),
    };

    Ok(Version {
        cli: cli_version?,
        client: client_version,
        engine: engine_version,
        plugin_buildx,
        plugin_compose,
    })
}

/// Get the versions from the `docker --version` command
pub(crate) async fn get_docker_cli_version(bin: &Executable) -> Result<semver::Version, Error> {
    let mut cmd = DockerCmd::with_executable(bin).get_version().into_command();
    let cmd_str = format!("{:?}", cmd.as_std());

    tracing::debug!("docker --version cmd: {}", cmd_str);

    let child = cmd
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    let output = tokio::time::timeout(Duration::from_secs(10), child.wait_with_output())
        .await
        .map_err(|_| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: anyhow::anyhow!("Command timed out").into(),
        })?
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    tracing::debug!("docker --version rc: {}", output.status);

    let output_str = String::from_utf8(output.stdout).map_err(|err| Error::OutputParsing {
        cmd: cmd_str.clone(),
        src: err.into(),
    })?;

    // Extract the version from the output, expected output is: `Docker version 28.0.4, build b8034c0ed7`. Note the `,` after the version.
    let version_str = output_str
        .strip_prefix("Docker version ")
        .and_then(|s| s.split(",").next())
        .ok_or(Error::OutputParsing {
            cmd: cmd_str.clone(),
            src: anyhow::anyhow!("Invalid output format: {}", output_str).into(),
        })?;

    let version = semver::Version::parse(version_str).map_err(|err| Error::VersionParsing {
        cmd: cmd_str,
        src: err,
    })?;

    Ok(version)
}

/// The versions retrieved from the `docker version` command
#[serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct DockerVersionVersions {
    /// The client version
    pub client: semver::Version,

    /// The engine version
    ///
    /// If the engine is not available, this will be `None`.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub engine: Option<semver::Version>,
}

/// Get the versions from the `docker version` command
pub(crate) async fn get_docker_version_versions(
    bin: &Executable,
) -> Result<DockerVersionVersions, Error> {
    // The go template to use to format the output of the `docker version` command into JSON
    const DOCKER_VERSION_FMT: &str = indoc::indoc! {
        r#"{
          "client":"{{.Client.Version}}",
          "client_api":"{{.Client.APIVersion}}"{{if .Server}},
          "engine":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Version}}{{end}}{{end}}",
          "engine_api":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Details.ApiVersion}}{{end}}{{end}}"{{end}}
        }"#
    };

    let mut cmd = DockerCmd::with_executable(bin)
        .version()
        .with_custom_format(DOCKER_VERSION_FMT)
        .into_command();
    let cmd_str = format!("{:?}", cmd.as_std());

    tracing::debug!("docker version cmd: {}", cmd_str);

    let child = cmd
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    let output = tokio::time::timeout(Duration::from_secs(10), child.wait_with_output())
        .await
        .map_err(|_| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: anyhow::anyhow!("Command timed out").into(),
        })?
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    tracing::debug!("docker version rc: {}", output.status);
    serde_json::from_slice(&output.stdout).map_err(|err| Error::OutputParsing {
        cmd: cmd_str,
        src: err.into(),
    })
}

/// The versions retrieved from the `docker system info` command
#[serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct DockerSystemInfoVersions {
    /// The version of the Docker compose plugin
    ///
    /// If the plugin is not installed, this will be `None`.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub plugin_compose: Option<semver::Version>,
    /// The version of the Docker buildx plugin
    ///
    /// If the plugin is not installed, this will be `None`.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub plugin_buildx: Option<semver::Version>,
}

/// Get the versions from the `docker system info` command
pub(crate) async fn get_docker_system_info_versions(
    bin: &Executable,
) -> Result<DockerSystemInfoVersions, Error> {
    // The go template to use to format the output of the `docker system info` command into JSON
    const DOCKER_SYSTEM_INFO_FMT: &str = indoc::indoc! {
        r#"{
          "plugin_compose":"{{range .ClientInfo.Plugins}}{{if eq .Name "compose"}}{{.Version}}{{end}}{{end}}",
          "plugin_buildx":"{{range .ClientInfo.Plugins}}{{if eq .Name "buildx"}}{{.Version}}{{end}}{{end}}"
        }"#
    };

    let mut cmd = DockerCmd::with_executable(bin)
        .system()
        .info()
        .with_custom_format(DOCKER_SYSTEM_INFO_FMT)
        .into_command();
    let cmd_str = format!("{:?}", cmd.as_std());

    tracing::debug!("docker system info cmd: {}", cmd_str);

    let child = cmd
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    let output = tokio::time::timeout(Duration::from_secs(10), child.wait_with_output())
        .await
        .map_err(|_| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: anyhow::anyhow!("Command timed out").into(),
        })?
        .map_err(|err| Error::CommandExecution {
            cmd: cmd_str.clone(),
            src: err.into(),
        })?;

    tracing::debug!("docker system info rc: {}", output.status);
    serde_json::from_slice(&output.stdout).map_err(|err| Error::OutputParsing {
        cmd: cmd_str,
        src: err.into(),
    })
}
