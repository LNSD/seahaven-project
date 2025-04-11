use super::common::IntoCommand;

pub struct JustVersionCmd {
    cmd: tokio::process::Command,
}

impl JustVersionCmd {
    /// Create a new `just --version` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
        }
    }
}

impl IntoCommand for JustVersionCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `version` subcommand
        cmd.arg("--version");

        cmd
    }
}
