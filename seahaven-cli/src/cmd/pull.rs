use std::{fs::File, io::BufReader, path::PathBuf};

use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use crate::result::{Error, Result};

/// The `pull` command name
pub const CMD: &str = "pull";

/// Create the `pull` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Pull the images for the development environment")
        .args([
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!([SERVICE] ... "The services to pull").action(clap::ArgAction::Append),
        ])
}

/// The `pull` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Check for requirements
    // - No min version for docker
    // - No min version for docker compose plugin (required)
    let docker_exe = seahaven_docker::exe::resolve_cli_executable()
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker executable: {err}"))?;

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

    let setup_file = File::open(setup_file)
        .map(BufReader::new)
        .map_err(|err| anyhow::anyhow!("Failed to open setup file: {err}"))?;
    let (env, content) = seahaven_file::from_reader(setup_file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?
        .unpack();

    // Transform the content into a compose file
    let compose = seahaven_file::try_into_compose_file(content)
        .map_err(|err| anyhow::anyhow!("Invalid setup file: {err}"))?;

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

    // Create and run the docker command
    let mut command = DockerCmd::with_executable(docker_exe)
        .compose()
        .with_file(content_file_path)
        .with_env_file(env_file_path)
        .with_plain_progress()
        .pull()
        .with_dry_run(matches.get_flag("dry-run"))
        .with_services(matches.get_many::<String>("SERVICE").unwrap_or_default())
        .into_command();

    tracing::debug!("command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn docker compose pull command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for docker compose pull command: {err}"))?;
    if !status.success() {
        return Err(
            Error::new(anyhow::anyhow!("Docker compose pull command failed"))
                .with_code(status.code().unwrap_or(1)),
        );
    }

    Ok(())
}
