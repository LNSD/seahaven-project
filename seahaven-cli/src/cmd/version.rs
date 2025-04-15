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
        ])
        .group(clap::ArgGroup::new("format").args(["short", "full", "json"]))
}

pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    let format = if matches.get_flag("short") {
        Format::Short
    } else if matches.get_flag("json") {
        Format::Json
    } else if matches.get_flag("full") {
        Format::Full
    } else {
        Format::Default
    };

    let info = get_build_info();
    match format {
        Format::Short => {
            println!("{}", info.crate_info.version);
        }
        Format::Default => {
            println!(
                "truman v{version} ({commit} {date})",
                version = info.crate_info.version,
                commit = format_build_info_commit_short_id(info),
                date = format_build_info_commit_date(info)
            );
        }
        Format::Json => {
            let info_json = serde_json::to_string(&info)
                .map_err(|err| anyhow::anyhow!("Failed to serialize build info: {}", err))?;
            println!("{}", info_json);
        }
        Format::Full => {
            indoc::printdoc! {
                r#"
                truman v{version}
                commit: {commit} ({commit_date})
                build:
                  profile: {build_profile}
                  date: {build_timestamp}
                  target: {build_target}
                  cpu: {build_target_cpu_arch} ({build_target_cpu_features})
                compiler:
                  version: rustc {compiler_version}
                  channel: {compiler_channel}
                  commit: {compiler_commit} ({compiler_commit_date})
                  host: {compiler_host_triple}
                "#,
                version=info.crate_info.version,
                commit=format_build_info_commit(info),
                commit_date=format_build_info_commit_date(info),
                build_profile=info.profile,
                build_timestamp=info.timestamp,
                build_target=info.target.triple,
                build_target_cpu_arch=info.target.cpu.arch,
                build_target_cpu_features=format_build_info_target_cpu_features(info),
                compiler_version=info.compiler.version,
                compiler_channel=info.compiler.channel,
                compiler_commit=format_build_info_compiler_commit(info),
                compiler_commit_date=format_build_info_compiler_commit_date(info),
                compiler_host_triple=info.compiler.host_triple,
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
