mod result;
mod setup;

use self::result::Result;

/// Create and execute the DIPs CLI command line interface
pub async fn run() -> Result<()> {
    let matches = clap::command!()
        .subcommands([setup::cmd()])
        .infer_long_args(true)
        .infer_subcommands(true)
        .get_matches();

    match matches.subcommand() {
        Some((setup::CMD, matches)) => setup::run(matches).await,
        _ => Err(anyhow::anyhow!("No command specified").into()),
    }
}
