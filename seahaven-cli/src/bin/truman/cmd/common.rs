//! Common options for all commands

use std::path::PathBuf;

/// The `-f`/`--file` argument
pub fn file_arg() -> clap::Arg {
    clap::arg!(-f --file <FILE> "The seahaven setup file")
        .default_value("setup.yaml")
        .value_parser(clap::value_parser!(PathBuf))
}

/// The `-e`/`--env-file` argument
pub fn env_file_arg() -> clap::Arg {
    clap::arg!(-e --"env-file" <ENV_FILE> "Specify an alternate environment file (can be specified multiple times)")
        .action(clap::ArgAction::Append)
        .value_parser(clap::value_parser!(PathBuf))
}
