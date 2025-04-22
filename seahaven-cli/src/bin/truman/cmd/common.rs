//! Common options for all commands

use std::path::PathBuf;

/// The `-f`/`--file` argument
pub fn file_arg() -> clap::Arg {
    clap::arg!(-f --file <FILE> "The seahaven setup file")
        .default_value("setup.yaml")
        .hide_default_value(true)
        .value_parser(clap::value_parser!(PathBuf))
}

/// The `--env-file` argument
pub fn env_file_arg() -> clap::Arg {
    clap::arg!(--"env-file" <ENV_FILE> "Specify an alternate environment file (can be specified multiple times)")
        .action(clap::ArgAction::Append)
        .value_parser(clap::value_parser!(PathBuf))
}

/// The `--project-directory` argument
pub fn project_directory_arg() -> clap::Arg {
    clap::arg!(--"project-directory" <PROJECT_DIRECTORY> "Specify an alternate project directory")
        .default_value(".")
        .hide_default_value(true)
        .value_parser(clap::value_parser!(PathBuf))
}
