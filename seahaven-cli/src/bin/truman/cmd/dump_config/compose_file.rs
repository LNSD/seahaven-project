use std::path::PathBuf;

use seahaven_cli::result::Result;

use crate::files::{
    into_compose_file, resolve_setup_file_and_project_dir_paths, setup_yaml::load_setup_file,
};

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
    let (setup_file, project_directory) = resolve_setup_file_and_project_dir_paths(
        matches
            .get_one::<PathBuf>("file")
            .expect("Failed to get setup file"),
        matches.get_one::<PathBuf>("project-directory"),
    )?;

    // Load the setup file and environment files
    let (_front_matter_env, content) = load_setup_file(&setup_file, &project_directory)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

    // Transform the content into a compose file
    let compose_file = into_compose_file(content);

    // Serialize the compose file to a string
    let compose_content = seahaven_compose_file::ser::to_string(&compose_file)
        .map_err(|err| anyhow::anyhow!("Failed to serialize compose file: {}", err))?;

    println!("{}", compose_content);

    Ok(())
}
