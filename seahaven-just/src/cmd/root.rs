use std::{
    borrow::Borrow,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use super::{
    common::{IntoCmdOptValue, IntoCommand},
    dump::JustDumpCmd,
    version::JustVersionCmd,
};
use crate::exe::{Executable, resolve_cli_executable};

pub struct JustCmd<F = JustfileNotSet, E = EnvFileNotSet> {
    cmd: tokio::process::Command,
    justfile_opt: F,
    env_file_opt: E,
}

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
        Self {
            cmd: tokio::process::Command::new(exe.borrow()),
            justfile_opt: JustfileNotSet,
            env_file_opt: EnvFileNotSet,
        }
    }
}

impl JustCmd {
    /// Create a new `just --version` command
    pub fn version(self) -> JustVersionCmd {
        JustVersionCmd::new(self)
    }

    /// Create a new `just --dump` command
    pub fn dump(self) -> JustDumpCmd {
        JustDumpCmd::new(self)
    }
}

impl<F, E> IntoCommand for JustCmd<F, E>
where
    F: JustfileOpt,
    E: EnvFileOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // --justfile <path>
        if let Some(justfile) = self.justfile_opt.into_value() {
            cmd.arg("--justfile").arg(justfile.as_ref());
        }

        // --dotenv-path <path>
        if let Some(env_file) = self.env_file_opt.into_value() {
            cmd.arg("--dotenv-path").arg(env_file.as_ref());
        }

        cmd
    }
}

impl<E> JustCmd<JustfileNotSet, E> {
    /// Specify an alternate justfile.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    /// for more information about the `--justfile` option.
    pub fn with_justfile<P>(self, file: P) -> JustCmd<JustfileSet, E>
    where
        P: AsRef<OsStr>,
    {
        let file = PathBuf::from(&file).into_boxed_path();

        JustCmd {
            cmd: self.cmd,
            justfile_opt: JustfileSet(file),
            env_file_opt: self.env_file_opt,
        }
    }
}

impl<F> JustCmd<F, EnvFileNotSet> {
    /// Specify an alternate environment file.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    /// for more information about the `--dotenv-path` option.
    pub fn with_env_file<P>(self, file: P) -> JustCmd<F, EnvFileSet>
    where
        P: AsRef<OsStr>,
    {
        let file = PathBuf::from(&file).into_boxed_path();

        JustCmd {
            cmd: self.cmd,
            justfile_opt: self.justfile_opt,
            env_file_opt: EnvFileSet(file),
        }
    }
}

// Justfile option markers
#[allow(private_bounds)]
pub trait JustfileOpt: IntoCmdOptValue<Box<Path>> + _priv::Sealed {}

pub struct JustfileNotSet;

impl JustfileOpt for JustfileNotSet {}
impl _priv::Sealed for JustfileNotSet {}

impl IntoCmdOptValue<Box<Path>> for JustfileNotSet {
    fn into_value(self) -> Option<Box<Path>> {
        None
    }
}

pub struct JustfileSet(Box<Path>);

impl JustfileOpt for JustfileSet {}
impl _priv::Sealed for JustfileSet {}

impl IntoCmdOptValue<Box<Path>> for JustfileSet {
    fn into_value(self) -> Option<Box<Path>> {
        Some(self.0)
    }
}

// Environment file markers
#[allow(private_bounds)]
pub trait EnvFileOpt: IntoCmdOptValue<Box<Path>> + _priv::Sealed {}

pub struct EnvFileNotSet;

impl EnvFileOpt for EnvFileNotSet {}
impl _priv::Sealed for EnvFileNotSet {}

impl IntoCmdOptValue<Box<Path>> for EnvFileNotSet {
    fn into_value(self) -> Option<Box<Path>> {
        None
    }
}

pub struct EnvFileSet(Box<Path>);

impl EnvFileOpt for EnvFileSet {}
impl _priv::Sealed for EnvFileSet {}

impl IntoCmdOptValue<Box<Path>> for EnvFileSet {
    fn into_value(self) -> Option<Box<Path>> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
