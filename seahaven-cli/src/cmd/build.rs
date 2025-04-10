use std::{fs::File, io::BufReader, path::PathBuf};

use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use crate::result::{Error, Result};

/// The `build` command name
pub(super) const CMD: &str = "build";

/// Create the `build` command
pub(super) fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Build the development environment images using docker compose")
        .args([
            clap::arg!(-f --file <FILE> "The seahaven setup file")
                .default_value("setup.yaml")
                .value_parser(clap::value_parser!(PathBuf)),
            clap::arg!(--dry-run "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--"build-arg" <KEY_VALUE> "Set build-time variables")
                .action(clap::ArgAction::Append)
                .value_parser(clap::builder::ValueParser::new(parse_build_arg)),
            clap::arg!([SERVICE] ... "The services to build").action(clap::ArgAction::Append),
        ])
}

/// The `build` command implementation
pub(super) async fn run(matches: &clap::ArgMatches) -> Result<()> {
    let setup_file = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file");
    if !setup_file.exists() {
        return Err(anyhow::anyhow!("Setup file not found: {}", setup_file.display()).into());
    }

    // Load the setup file
    let setup_file = File::open(setup_file)
        .map(BufReader::new)
        .map_err(|err| anyhow::anyhow!("Failed to open setup file: {err}"))?;
    let (env, content) = seahaven_file::from_reader(setup_file)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?
        .unpack();

    // Transform the content into a compose file
    let compose = seahaven_file::try_into_compose_file(content)
        .map_err(|err| anyhow::anyhow!("Invalid setup file: {err}"))?;

    // Create a tempfile for the env and the content
    let temp_dir = tempfile::Builder::new()
        .prefix("seahaven-")
        .tempdir()
        .map_err(|err| anyhow::anyhow!("Failed to create tempdir: {err}"))?;
    let temp_dir_path = temp_dir.path();
    tracing::debug!("Compose temporary directory: {}", temp_dir_path.display());

    let env_file_path = temp_dir_path.join(".env");
    let content_file_path = temp_dir_path.join("docker-compose.yaml");

    {
        let env_file = File::create(&env_file_path)
            .map_err(|err| anyhow::anyhow!("Failed to create .env file: {err}"))?;

        let content_file = File::create(&content_file_path)
            .map_err(|err| anyhow::anyhow!("Failed to create docker-compose.yaml file: {err}"))?;

        tracing::debug!("Writing .env file: {}", env_file_path.display());
        seahaven_file::serde_envfile::to_writer(env_file, &env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;

        tracing::debug!(
            "Writing docker-compose.yaml file: {}",
            content_file_path.display()
        );
        seahaven_file::seahaven_compose_file::ser::to_writer(content_file, &compose)
            .map_err(|err| anyhow::anyhow!("Failed to write docker-compose.yaml file: {err}"))?;
    }

    // Process and sanitize the build args
    let build_args = matches
        .get_many::<(String, String)>("build-arg")
        .unwrap_or_default()
        .fold(
            std::collections::BTreeMap::new(),
            |mut acc, (key, value)| {
                acc.insert(key, value);
                acc
            },
        );

    // Create and run the docker command
    let mut command = DockerCmd::new()
        .compose()
        .with_file(content_file_path)
        .with_env_file(env_file_path)
        .with_plain_progress()
        .build()
        .with_dry_run(matches.get_flag("dry-run"))
        .with_build_args(build_args)
        .with_services(matches.get_many::<String>("SERVICE").unwrap_or_default())
        .into_command();

    tracing::debug!("command: {:?}", command.as_std());

    let mut child = command
        .kill_on_drop(true)
        .spawn()
        .map_err(|err| anyhow::anyhow!("Failed to spawn docker compose build command: {err}"))?;

    let status = child
        .wait()
        .await
        .map_err(|err| anyhow::anyhow!("Failed to wait for docker compose build command: {err}"))?;
    if !status.success() {
        return Err(
            Error::new(anyhow::anyhow!("Docker compose build command failed"))
                .with_code(status.code().unwrap_or(1)),
        );
    }

    Ok(())
}

/// Parse a build arg
fn parse_build_arg(arg: &str) -> anyhow::Result<(String, String)> {
    if let Some((key, value)) = arg.split_once('=') {
        Ok((key.to_string(), value.to_string()))
    } else {
        Err(anyhow::anyhow!("Invalid build arg: {arg}"))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_build_arg;

    #[test]
    fn parse_build_arg_with_valid_input() {
        //* Given
        let input = "FOO=bar";

        //* When
        let result = parse_build_arg(input);

        //* Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("FOO".to_string(), "bar".to_string()));
    }

    #[test]
    fn parse_build_arg_with_empty_value() {
        //* Given
        let input = "FOO=";

        //* When
        let result = parse_build_arg(input);

        //* Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("FOO".to_string(), "".to_string()));
    }

    #[test]
    fn parse_build_arg_with_multiple_equals() {
        //* Given
        let input = "FOO=bar=baz";

        //* When
        let result = parse_build_arg(input);

        //* Then
        let (key, value) = result.expect("Failed to parse build arg");
        assert_eq!(key, "FOO");
        assert_eq!(value, "bar=baz");
    }

    #[test]
    fn parse_build_arg_with_no_equals() {
        //* Given
        let input = "FOO";

        //* When
        let result = parse_build_arg(input);

        //* Then
        let err = result.expect_err("Expected error");
        assert!(err.to_string().contains("Invalid build arg"));
    }

    #[test]
    fn parse_build_arg_with_empty_string() {
        //* Given
        let input = "";

        //* When
        let result = parse_build_arg(input);

        //* Then
        let err = result.expect_err("Expected error");
        assert!(err.to_string().contains("Invalid build arg"));
    }
}
