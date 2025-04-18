use super::common::IntoCommand;

pub struct JustListCmd(tokio::process::Command);

impl JustListCmd {
    /// Create a new `just --list` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self(cmd.into_command())
    }
}

impl IntoCommand for JustListCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.0;

        // Add the `list` subcommand
        cmd.arg("--list");

        cmd
    }
}
