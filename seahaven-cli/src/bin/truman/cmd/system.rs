use seahaven_cli::result::Result;

mod check;

/// The `system` command name
pub const CMD: &str = "system";

/// Create the `system` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Manage Seahaven")
        .subcommands([check::cmd()])
        .arg_required_else_help(true)
        .infer_long_args(true)
        .infer_subcommands(true)
}

/// The `system` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some((check::CMD, sub_matches)) => check::run(sub_matches).await,
        Some((cmd, _)) => unreachable!("unrecognized subcommand: {cmd}"),
        None => unreachable!("No subcommand specified"),
    }
}
