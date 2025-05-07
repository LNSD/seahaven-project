use std::{fs::File, path::PathBuf};

use seahaven_cli::result::{Error, Result};
use seahaven_just::cmd::{IntoCommand, JustCmd};

use super::common::{env_file_arg, file_arg, project_directory_arg};
use crate::{
    deps::resolve_just_executable,
    files::{
        env::{load_and_merge_envs, load_setup_file_env},
        resolve_setup_file_and_project_dir_paths,
    },
};

/// The `run` command name
pub const CMD: &str = "run";

/// Create the `run` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("run the development environment images using docker compose")
        .args([file_arg(), env_file_arg(), project_directory_arg()])
        .args([
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--justfile <FILE> "Path to the justfile to use")
                .value_parser(clap::value_parser!(PathBuf)),
            clap::arg!(--list "List available recipes")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("summary"),
            clap::arg!(--summary "Show summary of available recipes")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("list"),
            clap::arg!([ARGUMENTS] ... "Overrides and recipe(s) to run, defaulting to the first recipe in the justfile")
                .action(clap::ArgAction::Append)
                .conflicts_with_all(["list", "summary"]),
        ])
}

/// The `run` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Check for requirements
    // - No min version for just
    let just_exe = resolve_just_executable()
        .map_err(|err| anyhow::anyhow!("Failed to resolve just executable: {}", err))?;

    tracing::debug!("just executable: {}", just_exe);

    let just_version = seahaven_just::version::fetch(&just_exe)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to determine just version: {err}"))?;

    tracing::debug!("just version: {}", just_version);

    let (setup_file, project_directory) = resolve_setup_file_and_project_dir_paths(
        matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file"),
        matches.get_one::<PathBuf>("project-directory"),
    )?;

    let front_matter_env = load_setup_file_env(&setup_file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    // Create a tempfile for the env and the content
    let temp_dir = tempfile::Builder::new()
        .prefix("seahaven-")
        .tempdir()
        .map_err(|err| anyhow::anyhow!("Failed to create tempdir: {err}"))?;
    let temp_dir_path = temp_dir.path();
    tracing::debug!("Compose temporary directory: {}", temp_dir_path.display());

    let env_file_path = temp_dir_path.join(".env");

    {
        let env_file = File::create(&env_file_path)
            .map_err(|err| anyhow::anyhow!("Failed to create .env file: {err}"))?;

        tracing::debug!("Writing .env file: {}", env_file_path.display());
        serde_envfile::to_writer(env_file, &env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;
    }

    // Create and run the just command
    let mut command = if matches.get_flag("list") {
        JustCmd::with_executable(just_exe).list().into_command()
    } else if matches.get_flag("summary") {
        JustCmd::with_executable(just_exe).summary().into_command()
    } else {
        JustCmd::with_executable(just_exe)
            .with_justfile::<&PathBuf>(matches.get_one::<PathBuf>("justfile"))
            .with_working_directory(project_directory)
            .with_env_file(env_file_path)
            .with_dry_run(matches.get_flag("dry-run"))
            .with_args(matches.get_many::<String>("ARGUMENTS").unwrap_or_default())
            .into_command()
    };

    tracing::debug!("command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn just command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for just command: {err}"))?;
    if !status.success() {
        return Err(Error::new(anyhow::anyhow!("Just command failed"))
            .with_code(status.code().unwrap_or(1)));
    }

    Ok(())
}
