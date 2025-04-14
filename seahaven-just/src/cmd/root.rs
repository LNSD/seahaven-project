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

pub struct JustCmd<
    F = JustfileNotSet,
    E = EnvFileNotSet,
    W = WorkdirNotSet,
    D = DryRunNotSet,
    A = ArgsNotSet,
> {
    cmd: tokio::process::Command,
    justfile_opt: F,
    env_file_opt: E,
    workdir_opt: W,
    dry_run_opt: D,
    args_opt: A,
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
            workdir_opt: WorkdirNotSet,
            dry_run_opt: DryRunNotSet,
            args_opt: ArgsNotSet,
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

impl<F, E, W, D, A> IntoCommand for JustCmd<F, E, W, D, A>
where
    F: JustfileOpt,
    E: EnvFileOpt,
    W: WorkdirOpt,
    D: DryRunOpt,
    A: ArgsOpt,
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

        // --working-directory <path>
        if let Some(workdir) = self.workdir_opt.into_value() {
            cmd.arg("--working-directory").arg(workdir.as_ref());
        }

        // --dry-run
        if matches!(self.dry_run_opt.into_value(), Some(true)) {
            cmd.arg("--dry-run");
        }

        // [ARGUMENTS]...
        if let Some(args) = self.args_opt.into_value() {
            cmd.args(args);
        }

        cmd
    }
}

impl<E, W, D, A> JustCmd<JustfileNotSet, E, W, D, A> {
    /// Specify an alternate justfile.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    /// for more information about the `--justfile` option.
    pub fn with_justfile<P>(self, file: impl Into<Option<P>>) -> JustCmd<JustfileSet, E, W, D, A>
    where
        P: AsRef<OsStr>,
    {
        let file: Option<Box<Path>> = file
            .into()
            .map(|path| PathBuf::from(path.as_ref()).into_boxed_path());

        JustCmd {
            cmd: self.cmd,
            justfile_opt: JustfileSet(file),
            env_file_opt: self.env_file_opt,
            workdir_opt: self.workdir_opt,
            dry_run_opt: self.dry_run_opt,
            args_opt: self.args_opt,
        }
    }
}

impl<F, W, D, A> JustCmd<F, EnvFileNotSet, W, D, A> {
    /// Specify an alternate environment file.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    /// for more information about the `--dotenv-path` option.
    pub fn with_env_file<P>(self, file: P) -> JustCmd<F, EnvFileSet, W, D, A>
    where
        P: AsRef<OsStr>,
    {
        let file = PathBuf::from(&file).into_boxed_path();

        JustCmd {
            cmd: self.cmd,
            justfile_opt: self.justfile_opt,
            env_file_opt: EnvFileSet(file),
            workdir_opt: self.workdir_opt,
            dry_run_opt: self.dry_run_opt,
            args_opt: self.args_opt,
        }
    }
}

impl<E, D, A> JustCmd<JustfileSet, E, WorkdirNotSet, D, A> {
    /// Specify a working directory.
    ///
    /// Requires the `--justfile` option to be set.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    pub fn with_working_directory<P>(self, dir: P) -> JustCmd<JustfileSet, E, WorkdirSet, D, A>
    where
        P: AsRef<OsStr>,
    {
        let dir = PathBuf::from(&dir).into_boxed_path();

        JustCmd {
            cmd: self.cmd,
            justfile_opt: self.justfile_opt,
            env_file_opt: self.env_file_opt,
            workdir_opt: WorkdirSet(dir),
            dry_run_opt: self.dry_run_opt,
            args_opt: self.args_opt,
        }
    }
}

impl<F, E, W, A> JustCmd<F, E, W, DryRunNotSet, A> {
    /// Enable dry-run mode.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    pub fn with_dry_run(self, dry_run: bool) -> JustCmd<F, E, W, DryRunSet, A> {
        JustCmd {
            cmd: self.cmd,
            justfile_opt: self.justfile_opt,
            env_file_opt: self.env_file_opt,
            workdir_opt: self.workdir_opt,
            dry_run_opt: DryRunSet(dry_run),
            args_opt: self.args_opt,
        }
    }
}

impl<F, E, W, D> JustCmd<F, E, W, D, ArgsNotSet> {
    /// Add arguments to the command: overrides and recipe(s) to run.
    ///
    /// If empty, the first recipe in the justfile will be run.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    pub fn with_args<I>(self, args: I) -> JustCmd<F, E, W, D, ArgsSet>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let args = args.into_iter().map(|s| s.as_ref().to_string()).collect();

        JustCmd {
            cmd: self.cmd,
            justfile_opt: self.justfile_opt,
            env_file_opt: self.env_file_opt,
            workdir_opt: self.workdir_opt,
            dry_run_opt: self.dry_run_opt,
            args_opt: ArgsSet(args),
        }
    }

    /// Run the default recipe in the justfile.
    ///
    /// This is equivalent to calling the command with no arguments.
    ///
    /// See the [Just documentation](https://github.com/casey/just)
    pub fn run_default_recipe(self) -> JustCmd<F, E, W, D, ArgsSet> {
        self.with_args(Vec::<String>::new())
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

pub struct JustfileSet(Option<Box<Path>>);

impl JustfileOpt for JustfileSet {}
impl _priv::Sealed for JustfileSet {}

impl IntoCmdOptValue<Box<Path>> for JustfileSet {
    fn into_value(self) -> Option<Box<Path>> {
        self.0
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

// Workdir option markers
#[allow(private_bounds)]
pub trait WorkdirOpt: IntoCmdOptValue<Box<Path>> + _priv::Sealed {}

pub struct WorkdirNotSet;

impl WorkdirOpt for WorkdirNotSet {}
impl _priv::Sealed for WorkdirNotSet {}

impl IntoCmdOptValue<Box<Path>> for WorkdirNotSet {
    fn into_value(self) -> Option<Box<Path>> {
        None
    }
}

pub struct WorkdirSet(Box<Path>);

impl WorkdirOpt for WorkdirSet {}
impl _priv::Sealed for WorkdirSet {}

impl IntoCmdOptValue<Box<Path>> for WorkdirSet {
    fn into_value(self) -> Option<Box<Path>> {
        Some(self.0)
    }
}

// Dry run option markers
#[allow(private_bounds)]
pub trait DryRunOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct DryRunNotSet;

impl DryRunOpt for DryRunNotSet {}
impl _priv::Sealed for DryRunNotSet {}

impl IntoCmdOptValue<bool> for DryRunNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct DryRunSet(bool);

impl DryRunOpt for DryRunSet {}
impl _priv::Sealed for DryRunSet {}

impl IntoCmdOptValue<bool> for DryRunSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

// Args option markers
#[allow(private_bounds)]
pub trait ArgsOpt: IntoCmdOptValue<Box<[String]>> + _priv::Sealed {}

pub struct ArgsNotSet;

impl ArgsOpt for ArgsNotSet {}
impl _priv::Sealed for ArgsNotSet {}

impl IntoCmdOptValue<Box<[String]>> for ArgsNotSet {
    fn into_value(self) -> Option<Box<[String]>> {
        None
    }
}

pub struct ArgsSet(Box<[String]>);

impl ArgsOpt for ArgsSet {}
impl _priv::Sealed for ArgsSet {}

impl IntoCmdOptValue<Box<[String]>> for ArgsSet {
    fn into_value(self) -> Option<Box<[String]>> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
