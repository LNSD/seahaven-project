use std::borrow::Borrow;

use super::{
    common::IntoCommand, compose::DockerComposeCmd, system::DockerSystemCmd,
    version::DockerVersionCmd,
};
use crate::exe::{Executable, resolve_cli_executable};

pub struct DockerCmd(tokio::process::Command);

impl Default for DockerCmd {
    /// Create a new docker command
    ///
    /// # Panics
    ///
    /// This function will panic if the docker CLI binary is not found.
    fn default() -> Self {
        let exe = resolve_cli_executable().expect("Docker CLI binary not found");
        Self::with_executable(exe)
    }
}

impl DockerCmd {
    /// Create a new `docker` command
    ///
    /// This is equivalent to calling [`DockerCmd::default()`].
    ///
    /// # Panics
    ///
    /// This function will panic if the docker CLI binary is not found.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `docker` command with a custom executable
    pub fn with_executable<B>(exe: B) -> Self
    where
        B: Borrow<Executable>,
    {
        Self(tokio::process::Command::new(exe.borrow()))
    }
}

impl DockerCmd {
    /// Create a new `docker version` command
    pub fn version(self) -> DockerVersionCmd {
        DockerVersionCmd::new(self.0)
    }

    /// Create a new `docker system` command
    pub fn system(self) -> DockerSystemCmd {
        DockerSystemCmd::new(self.0)
    }

    /// Create a new `docker compose` command
    pub fn compose(self) -> DockerComposeCmd {
        DockerComposeCmd::new(self.0)
    }
}

impl IntoCommand for DockerCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}
