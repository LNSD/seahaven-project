//! This module contains the code for the `version` command.
//!
//! It is used to get the version of the docker CLI and the required plugins.

use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{DockerCmd, IntoCommand},
    exe::Executable,
};

/// The docker version
///
/// This struct is used to parse the output of the `docker version` command.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DockerVersion {
    /// The client version
    pub client: semver::Version,
    /// The engine version
    pub engine: semver::Version,
}

/// Get the version of the docker CLI
///
/// Run the `docker version` command and return the output.
pub async fn get_docker_version(bin: &Executable) -> anyhow::Result<DockerVersion> {
    // The go template to use to format the output of the `docker version` command into JSON
    const DOCKER_VERSION_FMT: &str = indoc::indoc! {
        r#"{
          "client":"{{.Client.Version}}",
          "client_api":"{{.Client.APIVersion}}",
          "engine":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Version}}{{end}}{{end}}",
          "engine_api":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Details.ApiVersion}}{{end}}{{end}}"
        }"#
    };

    let output = DockerCmd::with_binary(bin)
        .version()
        .with_custom_format(DOCKER_VERSION_FMT)
        .into_command()
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to run `docker version`: {}", err))?;

    serde_json::from_slice(&output.stdout)
        .map_err(|err| anyhow::anyhow!("Failed to parse `docker version` output: {}", err))
}

/// The version of the docker plugins
///
/// This struct is used to parse the output of the `docker system info` command.
#[serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DockerPluginVersions {
    /// The version of the docker compose plugin
    ///
    /// If the plugin is not installed, this will be `None`.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub compose: Option<semver::Version>,
    /// The version of the docker buildx plugin
    ///
    /// If the plugin is not installed, this will be `None`.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub buildx: Option<semver::Version>,
}

/// Get the `docker compose` version
///
/// Run the `docker compose version` command and return the output.
pub async fn get_docker_plugin_versions(bin: &Executable) -> anyhow::Result<DockerPluginVersions> {
    // The go template to use to format the output of the `docker system info` command into JSON
    const DOCKER_SYSTEM_INFO_FMT: &str = indoc::indoc! {
        r#"{
          "compose":"{{range .ClientInfo.Plugins}}{{if eq .Name "compose"}}{{.Version}}{{end}}{{end}}",
          "buildx":"{{range .ClientInfo.Plugins}}{{if eq .Name "buildx"}}{{.Version}}{{end}}{{end}}"
        }"#
    };

    let output = DockerCmd::with_binary(bin)
        .system()
        .info()
        .with_custom_format(DOCKER_SYSTEM_INFO_FMT)
        .into_command()
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to run `docker compose version`: {}", err))?;

    serde_json::from_slice(&output.stdout)
        .map_err(|err| anyhow::anyhow!("Failed to parse `docker compose version` output: {}", err))
}
