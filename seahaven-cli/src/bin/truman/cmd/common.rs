//! Common options for all commands

use std::path::PathBuf;

/// The `-f`/`--file` argument
pub fn file_arg() -> clap::Arg {
    clap::arg!(-f --file <FILE> "The seahaven setup file")
        .default_value("setup.yaml")
        .value_parser(clap::value_parser!(PathBuf))
}
