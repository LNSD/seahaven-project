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
        let bin = resolve_cli_executable().expect("Docker CLI binary not found");
        Self::with_executable(bin)
    }
}

impl DockerCmd {
    /// Create a new docker command
    ///
    /// This is equivalent to calling [`DockerCommand::default()`].
    ///
    /// # Panics
    ///
    /// This function will panic if the docker CLI binary is not found.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new docker command with a custom executable
    pub fn with_executable<B>(bin: B) -> Self
    where
        B: Borrow<Executable>,
    {
        Self(tokio::process::Command::new(bin.borrow()))
    }

    /// Create a new docker command with a custom executable
    #[cfg(test)]
    pub fn with_test_executable<E>(exe: E) -> Self
    where
        E: AsRef<std::ffi::OsStr>,
    {
        Self(tokio::process::Command::new(exe))
    }
}

impl DockerCmd {
    /// Create a new `docker version` command
    pub fn version(self) -> DockerVersionCmd {
        let mut cmd = self.0;
        cmd.arg("version");
        DockerVersionCmd::new(cmd)
    }

    /// Create a new `docker system` command
    pub fn system(self) -> DockerSystemCmd {
        let mut cmd = self.0;
        cmd.arg("system");
        DockerSystemCmd::new(cmd)
    }

    /// Create a new `docker compose` command
    pub fn compose(self) -> DockerComposeCmd {
        let mut cmd = self.0;
        cmd.arg("compose");
        DockerComposeCmd::new(cmd)
    }
}

impl IntoCommand for DockerCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}
