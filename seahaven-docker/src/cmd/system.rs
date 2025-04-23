pub mod info;
pub mod prune;

use self::{info::DockerSystemInfoCmd, prune::DockerSystemPruneCmd};
use super::common::IntoCommand;

pub struct DockerSystemCmd(tokio::process::Command);

impl DockerSystemCmd {
    /// Create a new `docker system` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self(cmd.into_command())
    }

    /// Create a new `docker system info` command
    pub fn info(self) -> DockerSystemInfoCmd {
        DockerSystemInfoCmd::new(self)
    }

    /// Create a new `docker system prune` command
    pub fn prune(self) -> DockerSystemPruneCmd {
        DockerSystemPruneCmd::new(self)
    }
}

impl IntoCommand for DockerSystemCmd {
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.0;

        // Add the `system` subcommand
        cmd.arg("system");

        cmd
    }
}
