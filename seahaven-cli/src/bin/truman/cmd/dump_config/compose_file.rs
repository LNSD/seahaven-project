use std::path::PathBuf;

use seahaven_cli::result::Result;

use crate::files::load_setup_file;

/// The `dump-config compose-file` command name
pub const CMD: &str = "compose-file";

/// Create the `dump-config compose-file` sub-command
pub fn cmd() -> clap::Command {
    clap::command!(CMD).about("Print the docker-compose.yaml file contents")
}

/// The `dump compose-file` command
///
/// This function prints the docker-compose.yaml file contents to the console.
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
    let (_front_matter_env, content) = load_setup_file(setup_file_path)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

    // Transform the content into a compose file
    let compose_file = seahaven_file::try_into_compose_file(content)
        .map_err(|err| anyhow::anyhow!("Failed to convert setup file to compose file: {}", err))?;

    // Serialize the compose file to a string
    let compose_content = seahaven_file::seahaven_compose_file::ser::to_string(&compose_file)
        .map_err(|err| anyhow::anyhow!("Failed to serialize compose file: {}", err))?;

    println!("{}", compose_content);

    Ok(())
}
