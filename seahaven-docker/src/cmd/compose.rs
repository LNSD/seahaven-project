use super::common::IntoCommand;

pub struct DockerComposeCmd(tokio::process::Command);

impl DockerComposeCmd {
    pub(crate) fn new(cmd: tokio::process::Command) -> Self {
        Self(cmd)
    }

    pub fn build(self) -> DockerComposeBuildCmd {
        let mut cmd = self.0;
        cmd.arg("build");
        DockerComposeBuildCmd(cmd)
    }

    pub fn up(self) -> DockerComposeUpCmd {
        let mut cmd = self.0;
        cmd.arg("up");
        DockerComposeUpCmd(cmd)
    }

    pub fn down(self) -> DockerComposeDownCmd {
        let mut cmd = self.0;
        cmd.arg("down");
        DockerComposeDownCmd(cmd)
    }
}

impl IntoCommand for DockerComposeCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub struct DockerComposeBuildCmd(tokio::process::Command);

impl IntoCommand for DockerComposeBuildCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub struct DockerComposeUpCmd(tokio::process::Command);

impl IntoCommand for DockerComposeUpCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub struct DockerComposeDownCmd(tokio::process::Command);

impl IntoCommand for DockerComposeDownCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub struct DockerComposePullCmd(tokio::process::Command);

impl IntoCommand for DockerComposePullCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}
