use std::{fs::File, io::BufReader, path::PathBuf};

use clap::{ArgMatches, Command, arg, command, value_parser};
use seahaven_file::compose::ComposeFile;

use super::result::Result;

/// The `setup` command name
pub(super) const CMD: &str = "setup";

/// Create the `setup` command
pub(super) fn cmd() -> Command {
    command!(CMD)
        .about("Manage local development environment setup")
        .subcommands([
            command!("env").about("Print the .env file contents").args([
                arg!(-f --file <FILE> "The setup file")
                    .default_value("setup.yaml")
                    .value_parser(value_parser!(PathBuf)),
                arg!(--"no-interpolation" "Do not interpolate the environment values"),
            ]),
            command!("compose")
                .about("Print the docker-compose.yaml file contents")
                .args([
                    arg!(-f --file <FILE> "The setup file")
                        .default_value("setup.yaml")
                        .value_parser(value_parser!(PathBuf)),
                    arg!(--"no-interpolation" "Do not interpolate the environment values"),
                ]),
            command!("eject")
                .about("Eject the setup.yaml file to get the docker-compose.yaml and .env files")
                .args([
                    arg!(-f --file <FILE> "The setup file" )
                        .default_value("setup.yaml")
                        .value_parser(value_parser!(PathBuf)),
                    arg!(-o --"output-dir" <DIR> "The output directory")
                        .default_value(".")
                        .value_parser(value_parser!(PathBuf)),
                    arg!(--"no-interpolation" "Do not interpolate the environment values"),
                ]),
        ])
}

/// The `setup` command implementation
pub(super) async fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("env", matches)) => env(matches).await,
        Some(("compose", matches)) => compose(matches).await,
        Some(("eject", matches)) => eject(matches).await,
        _ => Err(anyhow::anyhow!("No setup command specified").into()),
    }
}

/// The `setup env` command
///
/// This function prints the .env file contents to the console with all the interpolated values.
pub async fn env(matches: &ArgMatches) -> Result<()> {
    // Get the setup file path from the command line (required)
    let setup_file_path = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file path");
    if !setup_file_path.is_file() {
        return Err(anyhow::anyhow!("Invalid setup file: {}", setup_file_path.display()).into());
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

    // Check if we need to interpolate the environment variables
    if !matches.get_flag("no-interpolation") {
        // TODO: Implement interpolation (shellexpand crate?)
        eprintln!("\x1b[33m\x1b[1mWarning\x1b[0m: Env variables interpolation unavailable");
    }

    // Serialize the environment variables to a string
    let env_content = seahaven_file::serde_envfile::to_string(&env)
        .map_err(|err| anyhow::anyhow!("Failed to serialize environment variables: {}", err))?;

    println!("{}", env_content);

    Ok(())
}

/// The `setup compose` command
///
/// This function prints the docker-compose.yaml file contents to the console with all the interpolated values.
pub async fn compose(matches: &ArgMatches) -> Result<()> {
    // Get the setup file path from the command line (required)
    let setup_file_path = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file path");
    if !setup_file_path.is_file() {
        return Err(anyhow::anyhow!("Invalid setup file: {}", setup_file_path.display()).into());
    }

    // Read and parse the setup file
    let file = File::open(setup_file_path)
        .map(BufReader::new)
        .map_err(|err| anyhow::anyhow!("Failed to open setup file: {}", err))?;
    let (_env, content) = seahaven_file::from_reader(file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?
        .unpack();

    // Transform the content into a compose file
    // TODO: Move the transformation into the `seahaven_file` crate
    let compose_file = ComposeFile {
        name: content.name,
        services: content.services,
        networks: content.networks,
        volumes: content.volumes,
        configs: content.configs,
        secrets: content.secrets,
    };

    // Serialize the compose file to a string
    let compose_content = seahaven_file::compose::ser::to_string(&compose_file)
        .map_err(|err| anyhow::anyhow!("Failed to serialize compose file: {}", err))?;

    // Interpolate the compose file (env_file -> compose_file)
    if !matches.get_flag("no-interpolation") {
        // TODO: Implement interpolation (shellexpand crate?) for the compose file
        eprintln!("\x1b[33m\x1b[1mWarning\x1b[0m: Env variables interpolation unavailable");
    }

    println!("{}", compose_content);

    Ok(())
}

/// The `setup eject` command
///
/// This function ejects the setup.yaml file to get the docker-compose.yaml and .env files.
/// This is a one-way operation. Once ejected, you will need to manage the configuration manually.
pub async fn eject(matches: &ArgMatches) -> Result<()> {
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

    // Read and parse the setup file
    let file = File::open(setup_file_path)
        .map(BufReader::new)
        .map_err(|err| anyhow::anyhow!("Failed to open setup file: {}", err))?;
    let (env, content) = seahaven_file::from_reader(file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {}", err))?
        .unpack();

    // Transform the content into a compose file
    // TODO: Move the transformation into the `seahaven_file` crate
    let compose_file = ComposeFile {
        name: content.name,
        services: content.services,
        networks: content.networks,
        volumes: content.volumes,
        configs: content.configs,
        secrets: content.secrets,
    };

    // Serialize the compose file to a string
    let compose_content = seahaven_file::compose::ser::to_string(&compose_file)
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
        let env_content = seahaven_file::serde_envfile::to_string(&env)
            .map_err(|err| anyhow::anyhow!("Failed to serialize environment variables: {}", err))?;

        // Write the .env file
        let output_env_file_path = output_dir.join(".env");
        std::fs::write(&output_env_file_path, env_content)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {}", err))?;
        println!("Created .env file at {}", output_env_file_path.display());
    } else {
        println!("No environment variables found in the setup file, skipping .env file creation");
    }

    println!("Ejection completed successfully!");
    println!("You can now manage your configuration manually.");

    Ok(())
}
