use std::path::PathBuf;

use super::{build, down, init, pull, run, setup, up, version};
use crate::result::Result;

/// Create and execute the DIPs CLI command line interface
pub async fn cmd_run() -> Result<()> {
    let matches = clap::command!()
        .subcommands([
            build::cmd(),
            down::cmd(),
            init::cmd(),
            pull::cmd(),
            run::cmd(),
            setup::cmd(),
            up::cmd(),
            version::cmd(),
        ])
        .arg(
            clap::arg!(-f --file <FILE> "The seahaven setup file")
                .default_value("setup.yaml")
                .value_parser(clap::value_parser!(PathBuf))
                .global(true),
        )
        .infer_long_args(true)
        .infer_subcommands(true)
        .get_matches();

    match matches.subcommand() {
        Some((build::CMD, matches)) => build::run(matches).await,
        Some((down::CMD, matches)) => down::run(matches).await,
        Some((init::CMD, matches)) => init::run(matches).await,
        Some((pull::CMD, matches)) => pull::run(matches).await,
        Some((run::CMD, matches)) => run::run(matches).await,
        Some((setup::CMD, matches)) => setup::run(matches).await,
        Some((up::CMD, matches)) => up::run(matches).await,
        Some((version::CMD, matches)) => version::run(matches).await,
        _ => Err(anyhow::anyhow!("No command specified").into()),
    }
}
