//! Version information retrieval
//!
//! This module provides functionality to retrieve version details from different Docker components:
//! - The Docker client version (CLI)
//! - The Docker engine version (server)
//! - Docker plugins versions
//!
//! This version information is essential for ensuring compatibility and for troubleshooting issues.

use std::borrow::Borrow;

use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{DockerCmd, IntoCommand},
    exe::Executable,
};

/// The Docker version
///
/// This struct contains the version information of the Docker client, engine, and plugins.
///
/// See [`fetch`] for more information.
#[derive(Debug, Clone)]
pub struct Version {
    /// The client version
    pub client: semver::Version,
    /// The engine version
    pub engine: semver::Version,
    /// The compose plugin version
    ///
    /// If the plugin is not installed, this will be `None`.
    pub plugin_compose: Option<semver::Version>,
    /// The buildx plugin version
    ///
    /// If the plugin is not installed, this will be `None`.
    pub plugin_buildx: Option<semver::Version>,
}

/// Errors that can occur when retrieving Docker version information
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to execute the Docker command
    #[error("Failed to execute '{cmd}': {src}")]
    CommandExecution {
        cmd: &'static str,
        #[source]
        src: std::io::Error,
    },

    /// Failed to parse the Docker version output
    #[error("Failed to parse '{cmd}' output: {src}")]
    OutputParsing {
        cmd: &'static str,
        #[source]
        src: serde_json::Error,
    },

    /// Failed to parse a version string
    #[error("Failed to parse '{cmd}' version string: {src}")]
    VersionParsing {
        cmd: &'static str,
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
    let (version, system_info) = tokio::try_join!(
        get_docker_version_versions(bin.borrow()),
        get_docker_system_info_versions(bin.borrow())
    )?;

    Ok(Version {
        client: version.client,
        engine: version.engine,
        plugin_compose: system_info.plugin_compose,
        plugin_buildx: system_info.plugin_buildx,
    })
}

/// The versions retrieved from the `docker version` command
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct DockerVersionVersions {
    /// The client version
    pub client: semver::Version,
    /// The engine version
    pub engine: semver::Version,
}

/// Get the versions from the `docker version` command
pub(crate) async fn get_docker_version_versions(
    bin: &Executable,
) -> Result<DockerVersionVersions, Error> {
    // The go template to use to format the output of the `docker version` command into JSON
    const DOCKER_VERSION_FMT: &str = indoc::indoc! {
        r#"{
          "client":"{{.Client.Version}}",
          "client_api":"{{.Client.APIVersion}}",
          "engine":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Version}}{{end}}{{end}}",
          "engine_api":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Details.ApiVersion}}{{end}}{{end}}"
        }"#
    };

    let output = DockerCmd::with_executable(bin)
        .version()
        .with_custom_format(DOCKER_VERSION_FMT)
        .into_command()
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| Error::CommandExecution {
            cmd: "docker version",
            src: err,
        })?;

    serde_json::from_slice(&output.stdout).map_err(|err| Error::OutputParsing {
        cmd: "docker version",
        src: err,
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
          "plugin_compose":"{{range .ClientInfo.Plugins}}{{if eq .Name "compose"}}{{ if eq (slice .Version 0 1) "v" }}{{ slice .Version 1 }}{{ else }}{{ .Version }}{{ end }}{{end}}{{end}}",
          "plugin_buildx":"{{range .ClientInfo.Plugins}}{{if eq .Name "buildx"}}{{ if eq (slice .Version 0 1) "v" }}{{ slice .Version 1 }}{{ else }}{{ .Version }}{{ end }}{{end}}{{end}}"
        }"#
    };

    let output = DockerCmd::with_executable(bin)
        .system()
        .info()
        .with_custom_format(DOCKER_SYSTEM_INFO_FMT)
        .into_command()
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| Error::CommandExecution {
            cmd: "docker system info",
            src: err,
        })?;

    serde_json::from_slice(&output.stdout).map_err(|err| Error::OutputParsing {
        cmd: "docker system info",
        src: err,
    })
}
