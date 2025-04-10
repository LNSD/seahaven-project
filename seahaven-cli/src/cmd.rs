mod build;
mod pull;
mod setup;
mod up;

use crate::result::Result;

/// Create and execute the DIPs CLI command line interface
pub async fn run() -> Result<()> {
    let matches = clap::command!()
        .subcommands([setup::cmd(), up::cmd(), pull::cmd(), build::cmd()])
        .infer_long_args(true)
        .infer_subcommands(true)
        .get_matches();

    match matches.subcommand() {
        Some((setup::CMD, matches)) => setup::run(matches).await,
        Some((up::CMD, matches)) => up::run(matches).await,
        Some((pull::CMD, matches)) => pull::run(matches).await,
        Some((build::CMD, matches)) => build::run(matches).await,
        _ => Err(anyhow::anyhow!("No command specified").into()),
    }
}
