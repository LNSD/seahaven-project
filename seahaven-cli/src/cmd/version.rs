use build_info::BuildInfo;

use crate::result::Result;

// Generate the build info function.
build_info::build_info!(fn get_build_info);

pub const CMD: &str = "version";

pub fn cmd() -> clap::Command {
    clap::Command::new(CMD)
        .about("Print version information")
        .args([
            clap::arg!(--short "Print version information in short format")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--json "Print version information in JSON format")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--full "Print version information in full format")
                .action(clap::ArgAction::SetTrue),
            clap::arg!(--check "Check for dependency versions").action(clap::ArgAction::SetTrue),
        ])
        .group(clap::ArgGroup::new("format").args(["short", "full", "json", "check"]))
}

pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    // Get build info and dependency versions
    let info = {
        let build_info = get_build_info();
        let docker_version = fetch_docker_version().await;
        let just_version = fetch_just_version().await;

        info::into_version_info(build_info, docker_version, just_version)
    };

    // Check if dependencies are present
    if matches.get_flag("check") {
        return check_dependencies(&info);
    }

    let format = if matches.get_flag("short") {
        Format::Short
    } else if matches.get_flag("json") {
        Format::Json
    } else if matches.get_flag("full") {
        Format::Full
    } else {
        Format::Default
    };

    match format {
        Format::Default => {
            println!(
                "truman v{version} ({commit} {date})",
                version = info.version,
                commit = info.commit_short_id,
                date = info.commit_date
            );
        }
        Format::Short => {
            println!("{}", info.version);
        }
        Format::Json => {
            let info_json = serde_json::to_string_pretty(&info)
                .map_err(|err| anyhow::anyhow!("Failed to serialize version info: {}", err))?;
            println!("{}", info_json);
        }
        Format::Full => {
            indoc::printdoc! {
                r#"
                truman v{version}
                commit: {commit} ({commit_date})
                build:
                  profile: {build_profile}
                  date: {build_date}
                  target: {build_target}
                  cpu: {cpu_arch} ({cpu_features})
                compiler:
                  version: rustc {compiler_version}
                  channel: {compiler_channel}
                  commit: {compiler_commit} ({compiler_commit_date})
                  host: {compiler_host}
                dependencies:
                  docker:
                    cli: {docker_cli}
                    client: {docker_client}
                    engine: {docker_engine}
                    compose: {docker_compose}
                    buildx: {docker_buildx}
                  just: {just_version}
                "#,
                version=info.version,
                commit=info.commit,
                commit_date=info.commit_date,
                build_profile=info.build.profile,
                build_date=info.build.date,
                build_target=info.build.target,
                cpu_arch=info.build.cpu.arch,
                cpu_features=info.build.cpu.features,
                compiler_version=info.compiler.version,
                compiler_channel=info.compiler.channel,
                compiler_commit=info.compiler.commit,
                compiler_commit_date=info.compiler.commit_date,
                compiler_host=info.compiler.host,
                docker_cli=info.dependencies.docker.as_ref().map(|d| d.cli.clone()).unwrap_or("unknown".to_string()),
                docker_client=info.dependencies.docker.as_ref().and_then(|d| d.client.clone()).unwrap_or("unknown".to_string()),
                docker_engine=info.dependencies.docker.as_ref().and_then(|d| d.engine.clone()).unwrap_or("unknown".to_string()),
                docker_compose=info.dependencies.docker.as_ref().and_then(|d| d.compose.clone()).unwrap_or("unknown".to_string()),
                docker_buildx=info.dependencies.docker.as_ref().and_then(|d| d.buildx.clone()).unwrap_or("unknown".to_string()),
                just_version=info.dependencies.just.clone().unwrap_or("unknown".to_string()),
            };
        }
    }

    Ok(())
}

/// The format of the version information.
#[derive(Debug, Clone, Copy)]
enum Format {
    /// Default format.
    Default,
    /// Print version information in short format.
    Short,
    /// Print version information in full format.
    Full,
    /// Print version information in JSON format.
    Json,
}

/// Fetch the docker version
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

/// Fetch the just version
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

mod info {
    use super::*;

    #[derive(Debug, serde::Serialize)]
    pub struct VersionInfo {
        pub version: String,
        pub commit: String,
        pub commit_short_id: String,
        pub commit_date: String,
        pub build: BuildDetails,
        pub compiler: CompilerInfo,
        pub dependencies: Dependencies,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct BuildDetails {
        pub profile: String,
        pub date: String,
        pub target: String,
        pub cpu: CpuInfo,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct CpuInfo {
        pub arch: String,
        pub features: String,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct CompilerInfo {
        pub version: String,
        pub channel: String,
        pub commit: String,
        pub commit_date: String,
        pub host: String,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct Dependencies {
        pub docker: Option<DockerInfo>,
        pub just: Option<String>,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct DockerInfo {
        pub cli: String,
        pub client: Option<String>,
        pub engine: Option<String>,
        pub compose: Option<String>,
        pub buildx: Option<String>,
    }

    pub fn into_version_info(
        info: &BuildInfo,
        docker: Option<seahaven_docker::version::Version>,
        just: Option<seahaven_just::version::Version>,
    ) -> VersionInfo {
        VersionInfo {
            version: info.crate_info.version.to_string(),
            commit: format_build_info_commit(info),
            commit_short_id: format_build_info_commit_short_id(info),
            commit_date: format_build_info_commit_date(info),
            build: BuildDetails {
                profile: info.profile.clone(),
                date: info.timestamp.to_string(),
                target: info.target.triple.clone(),
                cpu: CpuInfo {
                    arch: info.target.cpu.arch.clone(),
                    features: format_build_info_target_cpu_features(info),
                },
            },
            compiler: CompilerInfo {
                version: info.compiler.version.to_string(),
                channel: info.compiler.channel.to_string(),
                commit: format_build_info_compiler_commit(info),
                commit_date: format_build_info_compiler_commit_date(info),
                host: info.compiler.host_triple.clone(),
            },
            dependencies: Dependencies {
                docker: docker.as_ref().map(|v| DockerInfo {
                    cli: v.cli.to_string(),
                    client: v.client.as_ref().map(|v| v.to_string()),
                    engine: v.engine.as_ref().map(|v| v.to_string()),
                    compose: v.plugin_compose.as_ref().map(|v| v.to_string()),
                    buildx: v.plugin_buildx.as_ref().map(|v| v.to_string()),
                }),
                just: just.as_ref().map(|v| v.to_string()),
            },
        }
    }

    /// Format the build commit hash (version_control.git.commit_short_id)
    fn format_build_info_commit_short_id(info: &BuildInfo) -> String {
        match info.version_control.as_ref().and_then(|vc| vc.git()) {
            Some(git) => {
                let hash = &git.commit_short_id;
                if git.dirty {
                    format!("{}-dirty", hash)
                } else {
                    hash.clone()
                }
            }
            None => "unknown".to_string(),
        }
    }

    /// Format the build commit hash (version_control.git.commit_id)
    fn format_build_info_commit(info: &BuildInfo) -> String {
        match info.version_control.as_ref().and_then(|vc| vc.git()) {
            Some(git) => {
                let hash = &git.commit_id;
                if git.dirty {
                    format!("{}-dirty", hash)
                } else {
                    hash.clone()
                }
            }
            None => "unknown".to_string(),
        }
    }

    /// Format the build commit date (version_control.git.commit_timestamp)
    fn format_build_info_commit_date(info: &BuildInfo) -> String {
        match info.version_control.as_ref().and_then(|vc| vc.git()) {
            // build-info has disabled the `chrono/alloc` feature, so we cannot use custom format
            Some(git) => git.commit_timestamp.to_string(),
            None => "unknown".to_string(),
        }
    }

    /// Format the compiler commit hash (compiler.commit_id)
    fn format_build_info_compiler_commit(info: &BuildInfo) -> String {
        match info.compiler.commit_id.as_ref() {
            Some(commit) => commit.clone(),
            None => "unknown".to_string(),
        }
    }

    /// Format the compiler commit date (compiler.commit_date)
    fn format_build_info_compiler_commit_date(info: &BuildInfo) -> String {
        match info.compiler.commit_date.as_ref() {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        }
    }

    /// Format the CPU features (target.cpu.features)
    fn format_build_info_target_cpu_features(info: &BuildInfo) -> String {
        info.target.cpu.features.join(", ")
    }
}

/// Check if all required dependencies are present
fn check_dependencies(info: &info::VersionInfo) -> Result<()> {
    let mut missing_deps = Vec::new();

    if let Some(docker) = info.dependencies.docker.as_ref() {
        if docker.client.is_none() {
            missing_deps.push("docker-client");
        }
        if docker.engine.is_none() {
            missing_deps.push("docker-engine");
        }
        if docker.compose.is_none() {
            missing_deps.push("docker-compose");
        }
        if docker.buildx.is_none() {
            missing_deps.push("docker-buildx");
        }
    } else {
        missing_deps.push("docker-cli");
    }

    if info.dependencies.just.is_none() {
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
