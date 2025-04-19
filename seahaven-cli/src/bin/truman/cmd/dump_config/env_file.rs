use std::path::PathBuf;

use seahaven_cli::result::Result;

use crate::files::{load_env_files, load_setup_file};

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
    // Get the setup file path from the command line (required)
    let setup_file_path = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file path");

    match setup_file_path.try_exists() {
        Ok(false) => {
            tracing::debug!(file=%setup_file_path.display(), "Setup file does not exist");
            return Err(anyhow::anyhow!(
                "Setup file '{}' does not exist",
                setup_file_path.display()
            )
            .into());
        }
        Err(err) => {
            tracing::debug!(file=%setup_file_path.display(), "Failed to check if setup file exists: {}", err);
            return Err(anyhow::anyhow!(
                "Invalid setup file '{}': {}",
                setup_file_path.display(),
                err
            )
            .into());
        }
        _ => (), // The file exists, and we can access it
    }

    // Check if the provided path is a regular file
    if !setup_file_path.is_file() {
        return Err(anyhow::anyhow!(
            "Invalid setup file '{}': not a file",
            setup_file_path.display()
        )
        .into());
    }

    // Load the setup file and environment files
    let (front_matter_env, _content) = load_setup_file(setup_file_path)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

    let files_env = load_env_files(matches.get_many::<PathBuf>("env-file").unwrap_or_default())?;

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

    // If no environment is present, print a warning and return
    let env = match env {
        Some(env) => env,
        None => {
            eprintln!("\x1b[33m\x1b[1mWarning\x1b[0m: No env found in the setup file");
            return Ok(());
        }
    };

    // Serialize the environment variables to a string
    let env_content = seahaven_file::serde_envfile::to_string(&env)
        .map_err(|err| anyhow::anyhow!("Failed to serialize environment variables: {}", err))?;

    println!("{}", env_content);

    Ok(())
}
