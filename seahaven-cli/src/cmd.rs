mod build;
mod down;
mod pull;
mod run;
mod setup;
mod up;

use crate::result::Result;

/// Create and execute the DIPs CLI command line interface
pub async fn run() -> Result<()> {
    let matches = clap::command!()
        .subcommands([
            build::cmd(),
            down::cmd(),
            pull::cmd(),
            run::cmd(),
            setup::cmd(),
            up::cmd(),
        ])
        .infer_long_args(true)
        .infer_subcommands(true)
        .get_matches();

    match matches.subcommand() {
        Some((build::CMD, matches)) => build::run(matches).await,
        Some((down::CMD, matches)) => down::run(matches).await,
        Some((pull::CMD, matches)) => pull::run(matches).await,
        Some((run::CMD, matches)) => run::run(matches).await,
        Some((setup::CMD, matches)) => setup::run(matches).await,
        Some((up::CMD, matches)) => up::run(matches).await,
        _ => Err(anyhow::anyhow!("No command specified").into()),
    }
}
