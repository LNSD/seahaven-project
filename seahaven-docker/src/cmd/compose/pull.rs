use super::IntoCommand;

pub struct DockerComposePullCmd(tokio::process::Command);

impl DockerComposePullCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self(cmd.into_command())
    }
}

impl IntoCommand for DockerComposePullCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.0;

        // Add the `pull` subcommand
        cmd.arg("pull");

        cmd
    }
}
