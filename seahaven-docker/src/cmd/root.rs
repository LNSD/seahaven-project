use std::borrow::Borrow;

use super::{
    common::IntoCommand, compose::DockerComposeCmd, root_version::DockerRootVersionCmd,
    system::DockerSystemCmd, version::DockerVersionCmd,
};
use crate::exe::Executable;

pub struct DockerCmd(tokio::process::Command);

impl DockerCmd {
    /// Create a new `docker` command with a custom executable
    pub fn with_executable<B>(exe: B) -> Self
    where
        B: Borrow<Executable>,
    {
        Self(tokio::process::Command::new(exe.borrow()))
    }
}

impl DockerCmd {
    /// Create a new `docker --version` command
    pub fn get_version(self) -> DockerRootVersionCmd {
        DockerRootVersionCmd::new(self)
    }

    /// Create a new `docker version` command
    pub fn version(self) -> DockerVersionCmd {
        DockerVersionCmd::new(self)
    }

    /// Create a new `docker system` command
    pub fn system(self) -> DockerSystemCmd {
        DockerSystemCmd::new(self)
    }

    /// Create a new `docker compose` command
    pub fn compose(self) -> DockerComposeCmd {
        DockerComposeCmd::new(self)
    }
}

impl IntoCommand for DockerCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}
