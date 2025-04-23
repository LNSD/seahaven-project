use seahaven_cli::result::Result;

use super::{build, down, dump_config, eject, init, logs, ps, pull, run, system, up, version};

/// The name of the CLI
pub const CMD: &str = "truman";

/// Create and execute the DIPs CLI command line interface
pub async fn cmd_run() -> Result<()> {
    let matches = clap::command!(CMD)
        .about("The CLI tool that helps you manage your local development environment setup with the ease of a well-scripted reality.")
        .long_about(indoc::indoc! {r#"
            The CLI tool that helps you manage your local development environment setup with the ease of a well-scripted reality.

            "𝑰𝒏 𝒄𝒂𝒔𝒆 𝑰 𝒅𝒐𝒏'𝒕 𝒔𝒆𝒆 𝒚𝒂, 𝒈𝒐𝒐𝒅 𝒂𝒇𝒕𝒆𝒓𝒏𝒐𝒐𝒏, 𝒈𝒐𝒐𝒅 𝒆𝒗𝒆𝒏𝒊𝒏𝒈, 𝒂𝒏𝒅 𝒈𝒐𝒐𝒅 𝒏𝒊𝒈𝒉𝒕!" — Truman Burbank
        "#})
        .subcommands([
            init::cmd(),
            build::cmd(),
            up::cmd(),
            down::cmd(),
            pull::cmd(),
            run::cmd(),
            logs::cmd(),
            ps::cmd(),
            eject::cmd(),
            dump_config::cmd(),
            system::cmd(),
            version::cmd(),
        ])
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .infer_long_args(true)
        .infer_subcommands(true)
        .get_matches();

    match matches.subcommand() {
        Some((build::CMD, matches)) => build::run(matches).await,
        Some((down::CMD, matches)) => down::run(matches).await,
        Some((eject::CMD, matches)) => eject::run(matches).await,
        Some((init::CMD, matches)) => init::run(matches).await,
        Some((logs::CMD, matches)) => logs::run(matches).await,
        Some((ps::CMD, matches)) => ps::run(matches).await,
        Some((pull::CMD, matches)) => pull::run(matches).await,
        Some((run::CMD, matches)) => run::run(matches).await,
        Some((dump_config::CMD, matches)) => dump_config::run(matches).await,
        Some((system::CMD, matches)) => system::run(matches).await,
        Some((up::CMD, matches)) => up::run(matches).await,
        Some((version::CMD, matches)) => version::run(matches).await,
        Some((cmd, _)) => unreachable!("Unrecognized subcommand '{cmd}'"),
        None => unreachable!("No subcommand specified"),
    }
}
