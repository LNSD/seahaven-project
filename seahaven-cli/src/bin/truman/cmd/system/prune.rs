use seahaven_cli::result::{Error, Result};
use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use crate::deps::resolve_docker_executable;

/// The `prune` command name
pub const CMD: &str = "prune";

/// Create the `prune` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD).about("Remove unused data").args([
        clap::arg!(-a --all "Remove all unused images not just dangling ones")
            .action(clap::ArgAction::SetTrue),
        clap::arg!(-f --force "Do not prompt for confirmation").action(clap::ArgAction::SetTrue),
        clap::arg!(--volumes "Prune anonymous volumes").action(clap::ArgAction::SetTrue),
    ])
}

/// The `prune` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Check for requirements
    let docker_exe = resolve_docker_executable()
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker executable: {}", err))?;

    tracing::debug!("docker executable: {}", docker_exe);

    let docker_version = seahaven_docker::version::fetch(&docker_exe)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to determine docker version: {err}"))?;

    tracing::debug!("docker version: {:?}", docker_version);

    let mut command = DockerCmd::with_executable(docker_exe)
        .system()
        .prune()
        .with_all(matches.get_flag("all"))
        .with_force(matches.get_flag("force"))
        .with_volumes(matches.get_flag("volumes"))
        .into_command();

    tracing::debug!("Running command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn docker system prune command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for docker system prune command: {err}"))?;
    if !status.success() {
        return Err(
            Error::new(anyhow::anyhow!("Docker system prune command failed"))
                .with_code(status.code().unwrap_or(1)),
        );
    }

    Ok(())
}
