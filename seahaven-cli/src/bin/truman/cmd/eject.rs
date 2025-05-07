use std::path::PathBuf;

use seahaven_cli::result::Result;

use super::common::{env_file_arg, file_arg, project_directory_arg};
use crate::files::{env::load_and_merge_envs, into_compose_file, load_setup_file};

/// The `eject` command name
pub const CMD: &str = "eject";

/// Create the `eject` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Eject the setup.yaml file to get the docker-compose.yaml and .env files")
        .args([file_arg(), env_file_arg(), project_directory_arg()])
        .args([clap::arg!(-o --"output-dir" <DIR> "The output directory")
            .default_value(".")
            .value_parser(clap::value_parser!(PathBuf))])
}

/// The `eject` command implementation
///
/// This function ejects the setup.yaml file to get the docker-compose.yaml and .env files.
/// This is a one-way operation. Once ejected, you will need to manage the configuration manually.
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Display warning message about the one-way nature of the operation
    indoc::eprintdoc! {r#"
        \x1b[33m\x1b[1mWARNING\x1b[0m: This is a one-way operation. Once ejected, you will need to manage the configuration manually.
        \x1b[33m\x1b[1mWARNING\x1b[0m: The ejected files will be created in the specified output directory.
        \x1b[33m\x1b[1mWARNING\x1b[0m: You can always regenerate the setup.yaml file from the ejected files if needed.
        \x1b[33m\x1b[1mWARNING\x1b[0m: Are you sure you want to continue? [y/N] "#
    }

    // Get user confirmation
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|err| anyhow::anyhow!("Failed to read user input: {err}"))?;
    if !input.trim().to_lowercase().starts_with('y') {
        eprintln!("Operation cancelled.");
        return Ok(());
    }

    // Resolve the project directory
    let project_directory = matches
        .get_one::<PathBuf>("project-directory")
        .expect("Failed to get project directory");

    tracing::debug!("Project directory: {}", project_directory.display());

    // Get the setup file path from the command line (required)
    let setup_file_path = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file path");
    if !setup_file_path.is_file() {
        return Err(anyhow::anyhow!("Invalid setup file: {}", setup_file_path.display()).into());
    }

    // Get the output directory from the command line (required)
    let output_dir = matches
        .get_one::<PathBuf>("output-dir")
        .expect("Failed to get output directory");
    if !output_dir.is_dir() {
        return Err(anyhow::anyhow!("Invalid output directory: {}", output_dir.display()).into());
    }

    // Load the setup file and environment files
    let (front_matter_env, content) = load_setup_file(setup_file_path, &output_dir)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    // Transform the content into a compose file
    let compose_file = into_compose_file(content);

    // Serialize the compose file to a string
    let compose_content = seahaven_compose_file::ser::to_string(&compose_file)
        .map_err(|err| anyhow::anyhow!("Failed to serialize compose file: {}", err))?;

    // Write the docker-compose.yaml file
    let output_compose_file_path = output_dir.join("docker-compose.yaml");
    std::fs::write(&output_compose_file_path, compose_content)
        .map_err(|err| anyhow::anyhow!("Failed to write docker-compose.yaml: {}", err))?;
    println!(
        "Created docker-compose.yaml file at {}",
        output_compose_file_path.display()
    );

    // If environment section is present, write the .env file
    if let Some(env) = env {
        // Serialize the environment variables to a string
        let env_content = serde_envfile::to_string(&env)
            .map_err(|err| anyhow::anyhow!("Failed to serialize environment variables: {}", err))?;

        // Write the .env file
        let output_env_file_path = output_dir.join(".env");
        std::fs::write(&output_env_file_path, env_content)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {}", err))?;
        println!("Created .env file at {}", output_env_file_path.display());
    } else {
        println!("No environment variables found in the setup file, skipping .env file creation");
    }

    println!("Setup ejected successfully!");
    println!("You can now manage your configuration manually.");

    Ok(())
}
