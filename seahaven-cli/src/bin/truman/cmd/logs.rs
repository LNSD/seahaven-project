use std::{fs::File, path::PathBuf};

use seahaven_cli::result::{Error, Result};
use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use super::common::{env_file_arg, file_arg, project_directory_arg};
use crate::{
    deps::resolve_docker_executable,
    files::{env::load_and_merge_envs, into_compose_file, load_setup_file},
};

/// The `logs` command name
pub const CMD: &str = "logs";

/// Create the `logs` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("View output from the containers")
        .args([file_arg(), env_file_arg(), project_directory_arg()])
        .args([
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--follow "Follow log output").action(clap::ArgAction::SetTrue),
            clap::arg!(--timestamps "Show timestamps").action(clap::ArgAction::SetTrue),
            clap::arg!([SERVICE] ... "The services to follow logs for")
                .action(clap::ArgAction::Append),
        ])
}

/// The `logs` command implementation
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

    // Resolve the project directory
    let project_directory = matches
        .get_one::<PathBuf>("project-directory")
        .expect("Failed to get project directory");

    tracing::debug!("Project directory: {}", project_directory.display());

    // Load the setup file
    let setup_file = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file");
    if !setup_file.exists() {
        return Err(anyhow::anyhow!("Setup file not found: {}", setup_file.display()).into());
    }

    let (front_matter_env, content) = load_setup_file(setup_file, &project_directory)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    // Transform the content into a compose file
    let compose = into_compose_file(content);

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
        serde_envfile::to_writer(env_file, &env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;

        tracing::debug!(
            "Writing docker-compose.yaml file: {}",
            content_file_path.display()
        );
        seahaven_compose_file::ser::to_writer(content_file, &compose)
            .map_err(|err| anyhow::anyhow!("Failed to write docker-compose.yaml file: {err}"))?;
    }

    let mut command = DockerCmd::with_executable(docker_exe)
        .compose()
        .with_file(content_file_path)
        .with_env_file(env_file_path)
        .with_project_directory(project_directory)
        .with_plain_progress()
        .logs()
        .with_dry_run(matches.get_flag("dry-run"))
        .with_follow(matches.get_flag("follow"))
        .with_timestamps(matches.get_flag("timestamps"))
        .with_services(matches.get_many::<String>("SERVICE").unwrap_or_default())
        .into_command();

    tracing::debug!("Running command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn docker compose logs command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for docker compose logs command: {err}"))?;
    if !status.success() {
        return Err(
            Error::new(anyhow::anyhow!("Docker compose logs command failed"))
                .with_code(status.code().unwrap_or(1)),
        );
    }

    Ok(())
}
