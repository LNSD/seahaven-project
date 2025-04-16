use std::{fs::File, io::BufReader, path::PathBuf};

use super::common::file_arg;
use crate::result::Result;
/// The `setup` command name
pub const CMD: &str = "setup";

/// Create the `setup` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Manage local development environment setup")
        .subcommands([env::cmd(), compose::cmd()])
        .arg(file_arg().global(true))
}

/// The `setup` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some((env::CMD, matches)) => env::run(matches).await,
        Some((compose::CMD, matches)) => compose::run(matches).await,
        _ => Err(anyhow::anyhow!("No setup command specified").into()),
    }
}

mod env {
    use super::*;

    /// The `setup env` command name
    pub const CMD: &str = "env";

    /// Create the `setup env` sub-command
    pub fn cmd() -> clap::Command {
        clap::command!(CMD).about("Print the .env file contents")
    }

    /// The `setup env` command
    ///
    /// This function prints the .env file contents to the console.
    pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
        // Get the setup file path from the command line (required)
        let setup_file_path = matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file path");
        if !setup_file_path.is_file() {
            return Err(
                anyhow::anyhow!("Invalid setup file: {}", setup_file_path.display()).into(),
            );
        }

        // Read and parse the setup file
        let file = File::open(setup_file_path)
            .map(BufReader::new)
            .map_err(|err| anyhow::anyhow!("Failed to open setup file: {}", err))?;
        let env = seahaven_file::fileenv_from_reader(file)
            .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

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
}

mod compose {
    use super::*;

    /// The `setup compose` command name
    pub const CMD: &str = "compose";

    /// Create the `setup compose` sub-command
    pub fn cmd() -> clap::Command {
        clap::command!(CMD).about("Print the docker-compose.yaml file contents")
    }

    /// The `setup compose` command
    ///
    /// This function prints the docker-compose.yaml file contents to the console.
    pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
        // Get the setup file path from the command line (required)
        let setup_file_path = matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file path");
        if !setup_file_path.is_file() {
            return Err(
                anyhow::anyhow!("Invalid setup file: {}", setup_file_path.display()).into(),
            );
        }

        // Read and parse the setup file
        let file = File::open(setup_file_path)
            .map(BufReader::new)
            .map_err(|err| anyhow::anyhow!("Failed to open setup file: {}", err))?;
        let (_env, content) = seahaven_file::from_reader(file)
            .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?
            .unpack();

        // Transform the content into a compose file
        let compose_file = seahaven_file::try_into_compose_file(content).map_err(|err| {
            anyhow::anyhow!("Failed to convert setup file to compose file: {}", err)
        })?;

        // Serialize the compose file to a string
        let compose_content =
            seahaven_file::seahaven_compose_file::ser::to_string(&compose_file)
                .map_err(|err| anyhow::anyhow!("Failed to serialize compose file: {}", err))?;

        println!("{}", compose_content);

        Ok(())
    }
}
