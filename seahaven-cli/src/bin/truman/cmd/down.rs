use std::path::PathBuf;

use seahaven_cli::result::{Error, Result};
use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use super::common::{env_file_arg, file_arg, project_directory_arg};
use crate::{
    deps::resolve_docker_executable,
    files::{
        env::load_and_merge_envs,
        into_compose_file, resolve_setup_file_and_project_dir_paths,
        setup_yaml::load_setup_file,
        tempdir::{self, HasComposeFilePath as _, HasEnvFilePath as _},
    },
};

/// The `down` command name
pub const CMD: &str = "down";

/// Create the `down` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Stop and remove containers, networks, images, and volumes")
        .args([file_arg(), env_file_arg(), project_directory_arg()])
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

    // Resolve the setup file and project directory paths
    let (setup_file, project_directory) = resolve_setup_file_and_project_dir_paths(
        matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file"),
        matches.get_one::<PathBuf>("project-directory"),
    )?;

    let (front_matter_env, content) = load_setup_file(&setup_file, &project_directory)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    let compose = into_compose_file(content);

    let temp = tempdir::new().create_dir()?.write_all(&env, &compose)?;

    let mut command = DockerCmd::with_executable(docker_exe)
        .compose()
        .with_file(temp.compose_file_path())
        .with_env_file(temp.env_file_path())
        .with_project_directory(project_directory)
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
