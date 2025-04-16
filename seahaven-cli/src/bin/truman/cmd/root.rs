use seahaven_cli::result::Result;

use super::{build, down, eject, init, pull, run, setup, system, up, version};

/// The name of the CLI
pub const CMD: &str = "truman";

/// Create and execute the DIPs CLI command line interface
pub async fn cmd_run() -> Result<()> {
    let matches = clap::command!(CMD)
        .about(indoc::indoc! {r#"
            The CLI tool that helps you manage your local development environment setup with the ease of a well-scripted reality.

            "𝑰𝒏 𝒄𝒂𝒔𝒆 𝑰 𝒅𝒐𝒏'𝒕 𝒔𝒆𝒆 𝒚𝒂, 𝒈𝒐𝒐𝒅 𝒂𝒇𝒕𝒆𝒓𝒏𝒐𝒐𝒏, 𝒈𝒐𝒐𝒅 𝒆𝒗𝒆𝒏𝒊𝒏𝒈, 𝒂𝒏𝒅 𝒈𝒐𝒐𝒅 𝒏𝒊𝒈𝒉𝒕!" — Truman Burbank
        "#})
        .subcommands([
            build::cmd(),
            down::cmd(),
            eject::cmd(),
            init::cmd(),
            pull::cmd(),
            run::cmd(),
            setup::cmd(),
            system::cmd(),
            up::cmd(),
            version::cmd(),
        ])
        .infer_long_args(true)
        .infer_subcommands(true)
        .disable_version_flag(true)
        .get_matches();

    match matches.subcommand() {
        Some((build::CMD, matches)) => build::run(matches).await,
        Some((down::CMD, matches)) => down::run(matches).await,
        Some((eject::CMD, matches)) => eject::run(matches).await,
        Some((init::CMD, matches)) => init::run(matches).await,
        Some((pull::CMD, matches)) => pull::run(matches).await,
        Some((run::CMD, matches)) => run::run(matches).await,
        Some((setup::CMD, matches)) => setup::run(matches).await,
        Some((system::CMD, matches)) => system::run(matches).await,
        Some((up::CMD, matches)) => up::run(matches).await,
        Some((version::CMD, matches)) => version::run(matches).await,
        _ => Err(anyhow::anyhow!("No command specified").into()),
    }
}
