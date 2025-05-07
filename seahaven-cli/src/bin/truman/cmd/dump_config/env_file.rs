use std::path::PathBuf;

use seahaven_cli::result::Result;

use crate::files::{
    env::{load_and_merge_envs, load_setup_file_env},
    resolve_setup_file_and_project_dir_paths,
};

/// The `dump-config env-file` command name
pub const CMD: &str = "env-file";

/// Create the `dump-config env-file` sub-command
pub fn cmd() -> clap::Command {
    clap::command!(CMD).about("Print the .env file contents")
}

/// The `dump env-file` command
///
/// This function prints the .env file contents to the console.
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    let (setup_file, project_directory) = resolve_setup_file_and_project_dir_paths(
        matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file"),
        matches.get_one::<PathBuf>("project-directory"),
    )?;

    // Load the setup file and environment files
    let front_matter_env = load_setup_file_env(&setup_file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    // If no environment is present, print a warning and return
    let env = match env {
        Some(env) => env,
        None => {
            eprintln!("\x1b[33m\x1b[1mWarning\x1b[0m: No env found in the setup file");
            return Ok(());
        }
    };

    // Serialize the environment variables to a string
    let env_content = serde_envfile::to_string(&env)
        .map_err(|err| anyhow::anyhow!("Failed to serialize environment variables: {}", err))?;

    println!("{}", env_content);

    Ok(())
}
