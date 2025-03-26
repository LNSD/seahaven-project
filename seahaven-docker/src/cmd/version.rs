use serde_with::{DisplayFromStr, serde_as};

use super::bin::CliBinary;

/// The go template to use to format the output of the `docker version` command into JSON
const DOCKER_VERSION_GO_TEMPLATE: &str = r#"{"client":"{{.Client.Version}}","client_api":"{{.Client.APIVersion}}","engine":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Version}}{{end}}{{end}}","engine_api":"{{range .Server.Components}}{{if eq .Name "Engine"}}{{.Details.ApiVersion}}{{end}}{{end}}"}"#;

/// The go template to use to format the output of the `docker system info` command into JSON
const DOCKER_SYSTEM_INFO_GO_TEMPLATE: &str = r#"{"compose":"{{range .ClientInfo.Plugins}}{{if eq .Name "compose"}}{{.Version}}{{end}}{{end}}","buildx":"{{range .ClientInfo.Plugins}}{{if eq .Name "buildx"}}{{.Version}}{{end}}{{end}}"}"#;

/// The version of the docker CLI
///
/// This struct is used to parse the output of the `docker version` command.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DockerVersion {
    /// The docker client version
    pub client: semver::Version,
    /// The docker engine version
    pub engine: semver::Version,
}

/// Get the version of the docker CLI
///
/// Run the `docker version` command and return the output.
pub async fn get_docker_version(bin: &CliBinary) -> anyhow::Result<DockerVersion> {
    let output = tokio::process::Command::new(bin)
        .arg("version")
        .arg("--format")
        .arg(DOCKER_VERSION_GO_TEMPLATE)
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
pub async fn get_docker_plugin_versions(bin: &CliBinary) -> anyhow::Result<DockerPluginVersions> {
    let output = tokio::process::Command::new(bin)
        .arg("system")
        .arg("info")
        .arg("--format")
        .arg(DOCKER_SYSTEM_INFO_GO_TEMPLATE)
        .kill_on_drop(true)
        .output()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to run `docker compose version`: {}", err))?;

    serde_json::from_slice(&output.stdout)
        .map_err(|err| anyhow::anyhow!("Failed to parse `docker compose version` output: {}", err))
}
