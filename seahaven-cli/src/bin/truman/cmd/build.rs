use std::{fs::File, path::PathBuf};

use seahaven_cli::result::{Error, Result};
use seahaven_docker::cmd::{DockerCmd, IntoCommand};

use super::common::{env_file_arg, file_arg, project_directory_arg};
use crate::{
    deps::resolve_docker_executable,
    files::{env::load_and_merge_envs, into_compose_file, load_setup_file},
};

/// The `build` command name
pub const CMD: &str = "build";

/// Create the `build` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Build the development environment images using docker compose")
        .args([file_arg(), env_file_arg(), project_directory_arg()])
        .args([
            clap::arg!(--"dry-run" "Execute command in dry run mode")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--"build-arg" <KEY_VALUE> "Set build-time variables")
                .action(clap::ArgAction::Append)
                .value_parser(clap::builder::ValueParser::new(parse_build_arg)),
            clap::arg!(--ssh [SSH_AUTH] "Set SSH authentications used when building service images (use 'default' for using your default SSH Agent)")
                .default_missing_value("default")
                .hide_default_value(true)
                .action(clap::ArgAction::Set),
            clap::arg!([SERVICE] ... "The services to build").action(clap::ArgAction::Append),
        ])
}

/// The `build` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Check for requirements
    // - No min version for docker
    // - No min version for docker compose plugin (required)
    // - No min version for buildx plugin (required)
    let docker_exe = resolve_docker_executable()
        .map_err(|err| anyhow::anyhow!("Failed to resolve docker executable: {}", err))?;

    tracing::debug!("docker executable: {}", docker_exe);

    let docker_version = seahaven_docker::version::fetch(&docker_exe)
        .await
        .map_err(|err| anyhow::anyhow!("Failed to determine docker version: {err}"))?;

    tracing::debug!("docker version: {:?}", docker_version);

    if docker_version.plugin_compose.is_none() {
        return Err(anyhow::anyhow!(
            "Failed to determine the docker compose plugin version. Is the docker compose plugin installed?"
        )
        .into());
    }
    if docker_version.plugin_buildx.is_none() {
        return Err(anyhow::anyhow!(
            "Failed to determine the buildx plugin version. Is the buildx plugin installed?"
        )
        .into());
    }

    // Resolve the project directory
    let project_directory = matches
        .get_one::<PathBuf>("project-directory")
        .expect("Failed to get project directory");

    tracing::debug!("Project directory: {}", project_directory.display());

    // Load the setup file
    let setup_file = matches
        .get_one::<PathBuf>("file")
        .expect("Failed to get setup file");
    if !setup_file.exists() {
        return Err(anyhow::anyhow!("Setup file not found: {}", setup_file.display()).into());
    }

    let (front_matter_env, content) = load_setup_file(setup_file, &project_directory)
        .map_err(|err| anyhow::anyhow!("Failed to parse setup file: {err}"))?;

    let env = load_and_merge_envs(
        matches.get_many::<PathBuf>("env-file"),
        &project_directory,
        front_matter_env,
    )?;

    // Transform the content into a compose file
    let compose = into_compose_file(content);

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
        serde_envfile::to_writer(env_file, &env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;

        tracing::debug!(
            "Writing docker-compose.yaml file: {}",
            content_file_path.display()
        );
        seahaven_compose_file::ser::to_writer(content_file, &compose)
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
    let mut command = DockerCmd::with_executable(docker_exe)
        .compose()
        .with_file(content_file_path)
        .with_env_file(env_file_path)
        .with_project_directory(project_directory)
        .with_plain_progress()
        .build()
        .with_dry_run(matches.get_flag("dry-run"))
        .with_build_args(build_args)
        .with_ssh_auth::<&String>(matches.get_one::<String>("ssh"))
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
