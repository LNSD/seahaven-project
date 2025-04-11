use std::borrow::Borrow;

use super::{common::IntoCommand, version::JustVersionCmd};
use crate::exe::{Executable, resolve_cli_executable};

pub struct JustCmd(tokio::process::Command);

impl Default for JustCmd {
    /// Create a new just command
    ///
    /// # Panics
    ///
    /// This function will panic if the just CLI binary is not found.
    fn default() -> Self {
        let exe = resolve_cli_executable().expect("Just CLI binary not found");
        Self::with_executable(exe)
    }
}

impl JustCmd {
    /// Create a new `just` command
    ///
    /// This is equivalent to calling [`JustCmd::default()`].
    ///
    /// # Panics
    ///
    /// This function will panic if the just CLI binary is not found.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `just` command with a custom executable
    pub fn with_executable<B>(exe: B) -> Self
    where
        B: Borrow<Executable>,
    {
        Self(tokio::process::Command::new(exe.borrow()))
    }
}

impl JustCmd {
    /// Create a new `just version` command
    pub fn version(self) -> JustVersionCmd {
        JustVersionCmd::new(self)
    }
}

impl IntoCommand for JustCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}
