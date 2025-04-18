use super::common::IntoCommand;

pub struct JustSummaryCmd(tokio::process::Command);

impl JustSummaryCmd {
    /// Create a new `just --summary` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self(cmd.into_command())
    }
}

impl IntoCommand for JustSummaryCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.0;

        // Add the `summary` subcommand
        cmd.arg("--summary");

        cmd
    }
}
