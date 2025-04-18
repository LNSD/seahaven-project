use seahaven_cli::result::Result;

/// The `check` command name
pub const CMD: &str = "check";

/// Create the `check` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Check system status")
        .args(
            [clap::arg!(--"dependencies" "Check for dependency versions")
                .action(clap::ArgAction::SetTrue)],
        )
        .group(clap::ArgGroup::new("check").args(["dependencies"]))
}

/// The `check` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    if matches.get_flag("dependencies") {
        check_dependencies().await?;
    } else {
        // Run all checks
        check_dependencies().await?;
    }

    Ok(())
}

/// Verifies the presence of required system dependencies including Docker components (client,
/// engine, compose plugin, buildx plugin) and Just CLI.
///
/// Returns an error with a list of missing dependencies if any are not found.
async fn check_dependencies() -> Result<()> {
    let docker_version = fetch_docker_version().await;
    let just_version = fetch_just_version().await;

    let mut missing_deps = Vec::new();

    // Check docker dependencies
    if let Some(docker) = docker_version {
        if docker.client.is_none() {
            missing_deps.push("docker-client");
        }
        if docker.engine.is_none() {
            missing_deps.push("docker-engine");
        }
        if docker.plugin_compose.is_none() {
            missing_deps.push("docker-compose");
        }
        if docker.plugin_buildx.is_none() {
            missing_deps.push("docker-buildx");
        }
    } else {
        missing_deps.push("docker-cli");
    }

    // Check just CLI dependency
    if just_version.is_none() {
        missing_deps.push("just-cli");
    }

    if !missing_deps.is_empty() {
        return Err(anyhow::anyhow!(
            "The following dependencies are missing: {}",
            missing_deps.join(", ")
        )
        .into());
    }

    Ok(())
}

/// Resolves the Docker CLI executable path and fetches version information for Docker components.
/// Returns None if Docker is not found or version check fails.
async fn fetch_docker_version() -> Option<seahaven_docker::version::Version> {
    let docker_exe = match seahaven_docker::exe::resolve_cli_executable() {
        Ok(exe) => exe,
        Err(err) => {
            tracing::debug!("Failed to resolve docker executable: {}", err);
            return None;
        }
    };

    match seahaven_docker::version::fetch(&docker_exe).await {
        Ok(version) => Some(version),
        Err(err) => {
            tracing::debug!("Failed to determine docker version: {}", err);
            None
        }
    }
}

/// Resolves the Just CLI executable path and fetches version information for Just.
/// Returns None if Just is not found or version check fails.
async fn fetch_just_version() -> Option<seahaven_just::version::Version> {
    let just_exe = match seahaven_just::exe::resolve_cli_executable() {
        Ok(exe) => exe,
        Err(err) => {
            tracing::debug!("Failed to resolve just executable: {}", err);
            return None;
        }
    };

    match seahaven_just::version::fetch(&just_exe).await {
        Ok(version) => Some(version),
        Err(err) => {
            tracing::debug!("Failed to determine just version: {}", err);
            None
        }
    }
}
