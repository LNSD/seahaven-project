use super::common::IntoCommand;

pub struct DockerRootVersionCmd(tokio::process::Command);

impl DockerRootVersionCmd {
    /// Create a new `docker --version` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self(cmd.into_command())
    }
}

impl IntoCommand for DockerRootVersionCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.0;

        // Add the `--version` flag
        cmd.arg("--version");

        cmd
    }
}
