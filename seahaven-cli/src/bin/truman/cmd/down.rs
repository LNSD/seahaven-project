use std::{fs::File, path::PathBuf};

use seahaven_cli::result::{Error, Result};
use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use super::common::{env_file_arg, file_arg};
use crate::{
    deps::resolve_docker_executable,
    files::{load_env_files, load_setup_file},
};

/// The `down` command name
pub const CMD: &str = "down";

/// Create the `down` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Stop and remove containers, networks, images, and volumes")
        .args([file_arg(), env_file_arg()])
        .args([
            clap::arg!(-v --volumes "Remove named volumes declared in the volumes section of the Compose file")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!([SERVICE] ... "The services to stop and remove")
                .action(clap::ArgAction::Append),
        ])
}

/// The `down` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Check for requirements
    // - No min version for docker
    // - No min version for docker compose plugin (required)
    let docker_exe = resolve_docker_executable()
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker executable: {}", err))?;

    tracing::debug!("docker executable: {}", docker_exe);

    let docker_version = seahaven_docker::version::fetch(&docker_exe)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to determine docker version: {err}"))?;

    tracing::debug!("docker version: {:?}", docker_version);

    if docker_version.plugin_compose.is_none() {
        return Err(anyhow::anyhow!(
            "Failed to determine the docker compose plugin version. Is the docker compose plugin installed?"
        )
        .into());
    }

    // Load the setup file
    let setup_file = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file");
    if !setup_file.exists() {
        return Err(anyhow::anyhow!("Setup file not found: {}", setup_file.display()).into());
    }

    let (front_matter_env, content) = load_setup_file(setup_file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?;

    let files_env = load_env_files(matches.get_many::<PathBuf>("env-file").unwrap_or_default())?;

    // Transform the content into a compose file
    let compose = seahaven_file::try_into_compose_file(content)
        .map_err(|err| anyhow::anyhow!("Invalid setup file: {err}"))?;

    // Merge the env files and front matter env
    let env = match (files_env, front_matter_env) {
        (Some(files_env), None) => Some(files_env),
        (None, Some(front_matter_env)) => Some(front_matter_env),
        (Some(files_env), Some(front_matter_env)) => {
            let mut env = files_env;
            env.extend(front_matter_env);
            Some(env)
        }
        (None, None) => None,
    };

    // Create a tempfile for the env and the content
    let temp_dir = tempfile::Builder::new()
        .prefix("seahaven-")
        .tempdir()
        .map_err(|err| anyhow::anyhow!("Failed to create tempdir: {err}"))?;
    let temp_dir_path = temp_dir.path();
    tracing::debug!("Compose temporary directory: {}", temp_dir_path.display());

    let env_file_path = temp_dir_path.join(".env");
    let content_file_path = temp_dir_path.join("docker-compose.yaml");

    {
        let env_file = File::create(&env_file_path)
            .map_err(|err| anyhow::anyhow!("Failed to create .env file: {err}"))?;

        let content_file = File::create(&content_file_path)
            .map_err(|err| anyhow::anyhow!("Failed to create docker-compose.yaml file: {err}"))?;

        tracing::debug!("Writing .env file: {}", env_file_path.display());
        seahaven_file::serde_envfile::to_writer(env_file, &env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;

        tracing::debug!(
            "Writing docker-compose.yaml file: {}",
            content_file_path.display()
        );
        seahaven_file::seahaven_compose_file::ser::to_writer(content_file, &compose)
            .map_err(|err| anyhow::anyhow!("Failed to write docker-compose.yaml file: {err}"))?;
    }

    let mut command = DockerCmd::with_executable(docker_exe)
        .compose()
        .with_file(content_file_path)
        .with_env_file(env_file_path)
        .with_plain_progress()
        .down()
        .with_volumes(matches.get_flag("volumes"))
        .with_dry_run(matches.get_flag("dry-run"))
        .with_services(matches.get_many::<String>("SERVICE").unwrap_or_default())
        .into_command();

    tracing::debug!("Running command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn docker compose down command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for docker compose down command: {err}"))?;
    if !status.success() {
        return Err(
            Error::new(anyhow::anyhow!("Docker compose down command failed"))
                .with_code(status.code().unwrap_or(1)),
        );
    }

    Ok(())
}
