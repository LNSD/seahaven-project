use seahaven_cli::result::Result;

use super::common::{env_file_arg, file_arg, project_directory_arg};

mod compose_file;
mod env_file;

/// The `dump-config` command name
pub const CMD: &str = "dump-config";

/// Create the `dump-config` command
pub fn cmd() -> clap::Command {
    clap::command!(CMD)
        .about("Dump the project's configuration")
        .long_about(indoc::indoc! {r#"
            Dump the project's configuration

            This command allows you to inspect and use the project's configuration files. It's useful for:
            - Debugging configuration issues
            - Verifying configuration values
            - Using the configuration with other tools
            - Understanding how the project is configured

            Some examples:

            Inspecting the environment configuration:

              truman dump-config env-file | grep -E '^(CHAIN_RPC|CHAIN_ID)'

            Inspecting the Docker Compose configuration:

              truman dump-config compose-file | yq '.services["chain"].ports'

            Saving the configuration to files:

              truman dump-config env-file > .env
              truman dump-config compose-file > docker-compose.yml

            Using the configuration with the just task runner command:

              just --dotenv-path <(truman dump-config env-file) mine-blocks

            Using the configuration with the Docker Compose command; this would be equivalent to running `truman up`:

              docker compose --file <(truman dump-config compose-file) --env-file <(truman dump-config env-file) --project-directory=$PWD up
        "#})
        .subcommands([env_file::cmd(), compose_file::cmd()])
        .args([file_arg().global(true), env_file_arg().global(true), project_directory_arg().global(true)])
        .arg_required_else_help(true)
        .subcommand_required(true)
        .infer_long_args(true)
        .infer_subcommands(true)
}

/// The `dump-config` command implementation
pub async fn run(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some((env_file::CMD, sub_matches)) => env_file::run(sub_matches).await,
        Some((compose_file::CMD, sub_matches)) => compose_file::run(sub_matches).await,
        Some((cmd, _)) => unreachable!("unrecognized subcommand: {cmd}"),
        None => unreachable!("No subcommand specified"),
    }
}
