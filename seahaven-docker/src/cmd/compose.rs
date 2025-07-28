use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub mod build;
pub mod down;
pub mod logs;
pub mod ps;
pub mod pull;
pub mod restart;
pub mod start;
pub mod stop;
pub mod up;

use self::{
    build::DockerComposeBuildCmd, down::DockerComposeDownCmd, logs::DockerComposeLogsCmd,
    ps::DockerComposePsCmd, pull::DockerComposePullCmd, restart::DockerComposeRestartCmd,
    start::DockerComposeStartCmd, stop::DockerComposeStopCmd, up::DockerComposeUpCmd,
};
use super::common::{IntoCmdOptValue, IntoCommand};

pub struct DockerComposeCmd<
    N = NameNotSet,
    F = ProjectFileNotSet,
    D = ProjectDirNotSet,
    E = EnvFileNotSet,
    P = ProgressNotSet,
    A = AnsiNotSet,
> {
    cmd: tokio::process::Command,
    name_opt: N,
    project_file_opt: F,
    project_dir_opt: D,
    env_file_opt: E,
    progress_opt: P,
    ansi_opt: A,
}

impl DockerComposeCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            name_opt: NameNotSet,
            project_file_opt: ProjectFileNotSet,
            project_dir_opt: ProjectDirNotSet,
            env_file_opt: EnvFileNotSet,
            progress_opt: ProgressNotSet,
            ansi_opt: AnsiNotSet,
        }
    }
}

impl<N, F, D, E, P, A> DockerComposeCmd<N, F, D, E, P, A>
where
    N: NameOpt,
    F: ProjectFileOpt,
    D: ProjectDirOpt,
    E: EnvFileOpt,
    P: ProgressOpt,
    A: AnsiOpt,
{
    /// Pull the images for the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose pull`](https://docs.docker.com/compose/reference/pull/)
    /// for more information.
    pub fn pull(self) -> DockerComposePullCmd {
        DockerComposePullCmd::new(self)
    }

    /// Build the images for the services defined in the `docker-compose.yml` file.
    ///
    /// See [`Docker Compose Build`](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn build(self) -> DockerComposeBuildCmd {
        DockerComposeBuildCmd::new(self)
    }

    /// Start the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose start`](https://docs.docker.com/compose/reference/start/)
    /// for more information.
    pub fn start(self) -> DockerComposeStartCmd {
        DockerComposeStartCmd::new(self)
    }

    /// Stop the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose stop`](https://docs.docker.com/compose/reference/stop/)
    /// for more information.
    pub fn stop(self) -> DockerComposeStopCmd {
        DockerComposeStopCmd::new(self)
    }

    /// Restart the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose restart`](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn restart(self) -> DockerComposeRestartCmd {
        DockerComposeRestartCmd::new(self)
    }

    /// Start the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose up`](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn up(self) -> DockerComposeUpCmd {
        DockerComposeUpCmd::new(self)
    }

    /// Stop the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose down`](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn down(self) -> DockerComposeDownCmd {
        DockerComposeDownCmd::new(self)
    }

    /// View output from containers.
    ///
    /// See [`docker compose logs`](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn logs(self) -> DockerComposeLogsCmd {
        DockerComposeLogsCmd::new(self)
    }

    /// List containers for the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose ps`](https://docs.docker.com/compose/reference/ps/)
    /// for more information.
    pub fn ps(self) -> DockerComposePsCmd {
        DockerComposePsCmd::new(self)
    }
}

impl<N, F, D, E, P, A> IntoCommand for DockerComposeCmd<N, F, D, E, P, A>
where
    N: NameOpt,
    F: ProjectFileOpt,
    D: ProjectDirOpt,
    E: EnvFileOpt,
    P: ProgressOpt,
    A: AnsiOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `compose` subcommand
        cmd.arg("compose");

        // --project-name <name>
        if let Some(name) = self.name_opt.into_value() {
            cmd.arg("--project-name").arg(name);
        }

        // --file <path>
        if let Some(file) = self.project_file_opt.into_value() {
            cmd.arg("--file").arg(file.as_os_str());
        }

        // --project-directory <path>
        if let Some(project_dir) = self.project_dir_opt.into_value() {
            cmd.arg("--project-directory").arg(project_dir.as_os_str());
        }

        // --env-file <path>
        if let Some(env_file) = self.env_file_opt.into_value() {
            cmd.arg("--env-file").arg(env_file.as_os_str());
        }

        // --progress <type>
        if let Some(progress) = self.progress_opt.into_value() {
            cmd.arg("--progress").arg(progress.to_string());
        }

        // --ansi <type>
        if let Some(ansi) = self.ansi_opt.into_value() {
            cmd.arg("--ansi").arg(ansi.to_string());
        }

        cmd
    }
}

impl<F, D, E, P, A> DockerComposeCmd<NameNotSet, F, D, E, P, A> {
    /// Specify a project name for the Docker Compose deployment.
    ///
    /// The project name is used as a prefix for container names and creates an isolated
    /// environment for the services. By default, Docker Compose uses the directory name
    /// of the compose file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/#use--p-to-specify-a-project-name)
    /// for more information about the `--project-name` option.
    pub fn with_project_name<S>(self, name: S) -> DockerComposeCmd<NameSet, F, D, E, P, A>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref().to_string();

        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: NameSet(name),
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: self.ansi_opt,
        }
    }
}

impl<N, D, E, P, A> DockerComposeCmd<N, ProjectFileNotSet, D, E, P, A> {
    /// Specify an alternate compose file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--file` option.
    pub fn with_file<S>(self, file: S) -> DockerComposeCmd<N, ProjectFileSet, D, E, P, A>
    where
        S: AsRef<OsStr>,
    {
        let file = PathBuf::from(&file).into_boxed_path();

        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: ProjectFileSet(file),
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: self.ansi_opt,
        }
    }
}

impl<N, F, E, P, A> DockerComposeCmd<N, F, ProjectDirNotSet, E, P, A> {
    /// Specify the project directory.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--project-directory` option.
    pub fn with_project_directory<S>(
        self,
        project_dir: S,
    ) -> DockerComposeCmd<N, F, ProjectDirSet, E, P, A>
    where
        S: AsRef<OsStr>,
    {
        let project_dir = PathBuf::from(&project_dir).into_boxed_path();

        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: ProjectDirSet(project_dir),
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: self.ansi_opt,
        }
    }
}

impl<N, F, D, P, A> DockerComposeCmd<N, F, D, EnvFileNotSet, P, A> {
    /// Specify an alternate environment file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--env-file` option.
    pub fn with_env_file<T>(self, env_file: T) -> DockerComposeCmd<N, F, D, EnvFileSet, P, A>
    where
        T: AsRef<OsStr>,
    {
        let file = PathBuf::from(&env_file).into_boxed_path();

        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: EnvFileSet(file),
            progress_opt: self.progress_opt,
            ansi_opt: self.ansi_opt,
        }
    }
}

impl<N, F, D, E, A> DockerComposeCmd<N, F, D, E, ProgressNotSet, A> {
    /// Set the type of progress output to auto (default).
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_auto_progress(self) -> DockerComposeCmd<N, F, D, E, ProgressSet, A> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: ProgressSet(Progress::Auto),
            ansi_opt: self.ansi_opt,
        }
    }

    /// Set the type of progress output to TTY
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_tty_progress(self) -> DockerComposeCmd<N, F, D, E, ProgressSet, A> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: ProgressSet(Progress::Tty),
            ansi_opt: self.ansi_opt,
        }
    }

    /// Set the type of progress output to plain text
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_plain_progress(self) -> DockerComposeCmd<N, F, D, E, ProgressSet, A> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: ProgressSet(Progress::Plain),
            ansi_opt: self.ansi_opt,
        }
    }

    /// Set the type of progress output to JSON
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_json_progress(self) -> DockerComposeCmd<N, F, D, E, ProgressSet, A> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: ProgressSet(Progress::Json),
            ansi_opt: self.ansi_opt,
        }
    }

    /// Set the type of progress output to quiet (no output)
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_quiet_progress(self) -> DockerComposeCmd<N, F, D, E, ProgressSet, A> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: ProgressSet(Progress::Quiet),
            ansi_opt: self.ansi_opt,
        }
    }
}

impl<N, F, D, E, P> DockerComposeCmd<N, F, D, E, P, AnsiNotSet> {
    /// Automatically detect whether to print ANSI control characters (default).
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/)
    /// for more information about the `--ansi` option.
    pub fn with_ansi_auto(self) -> DockerComposeCmd<N, F, D, E, P, AnsiSet> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: AnsiSet(Ansi::Auto),
        }
    }

    /// Always print ANSI control characters.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/)
    /// for more information about the `--ansi` option.
    pub fn with_ansi_always(self) -> DockerComposeCmd<N, F, D, E, P, AnsiSet> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: AnsiSet(Ansi::Always),
        }
    }

    /// Never print ANSI control characters.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/)
    /// for more information about the `--ansi` option.
    pub fn with_ansi_never(self) -> DockerComposeCmd<N, F, D, E, P, AnsiSet> {
        DockerComposeCmd {
            cmd: self.cmd,
            name_opt: self.name_opt,
            project_file_opt: self.project_file_opt,
            project_dir_opt: self.project_dir_opt,
            env_file_opt: self.env_file_opt,
            progress_opt: self.progress_opt,
            ansi_opt: AnsiSet(Ansi::Never),
        }
    }
}

// Project name markers
#[allow(private_bounds)]
pub trait NameOpt: IntoCmdOptValue<String> + _priv::Sealed {}

pub struct NameNotSet;

impl NameOpt for NameNotSet {}
impl _priv::Sealed for NameNotSet {}

impl IntoCmdOptValue<String> for NameNotSet {
    fn into_value(self) -> Option<String> {
        None
    }
}

pub struct NameSet(String);

impl NameOpt for NameSet {}
impl _priv::Sealed for NameSet {}

impl IntoCmdOptValue<String> for NameSet {
    fn into_value(self) -> Option<String> {
        Some(self.0)
    }
}

// Project file markers
#[allow(private_bounds)]
pub trait ProjectFileOpt: IntoCmdOptValue<Box<Path>> + _priv::Sealed {}

pub struct ProjectFileNotSet;

impl ProjectFileOpt for ProjectFileNotSet {}
impl _priv::Sealed for ProjectFileNotSet {}

impl IntoCmdOptValue<Box<Path>> for ProjectFileNotSet {
    fn into_value(self) -> Option<Box<Path>> {
        None
    }
}

pub struct ProjectFileSet(Box<Path>);

impl ProjectFileOpt for ProjectFileSet {}
impl _priv::Sealed for ProjectFileSet {}

impl IntoCmdOptValue<Box<Path>> for ProjectFileSet {
    fn into_value(self) -> Option<Box<Path>> {
        Some(self.0)
    }
}

// Project directory markers
#[allow(private_bounds)]
pub trait ProjectDirOpt: IntoCmdOptValue<Box<Path>> + _priv::Sealed {}

pub struct ProjectDirNotSet;

impl ProjectDirOpt for ProjectDirNotSet {}
impl _priv::Sealed for ProjectDirNotSet {}

impl IntoCmdOptValue<Box<Path>> for ProjectDirNotSet {
    fn into_value(self) -> Option<Box<Path>> {
        None
    }
}

pub struct ProjectDirSet(Box<Path>);

impl ProjectDirOpt for ProjectDirSet {}
impl _priv::Sealed for ProjectDirSet {}

impl IntoCmdOptValue<Box<Path>> for ProjectDirSet {
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

// Progress output type
#[derive(Debug, Clone, Copy, Default)]
pub enum Progress {
    #[default]
    Auto,
    Tty,
    Plain,
    Json,
    Quiet,
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Tty => "tty",
            Self::Plain => "plain",
            Self::Json => "json",
            Self::Quiet => "quiet",
        };
        write!(f, "{s}")
    }
}

// Progress output type markers
#[allow(private_bounds)]
pub trait ProgressOpt: IntoCmdOptValue<Progress> + _priv::Sealed {}

pub struct ProgressNotSet;

impl ProgressOpt for ProgressNotSet {}
impl _priv::Sealed for ProgressNotSet {}

impl IntoCmdOptValue<Progress> for ProgressNotSet {
    fn into_value(self) -> Option<Progress> {
        None
    }
}

pub struct ProgressSet(Progress);

impl ProgressOpt for ProgressSet {}
impl _priv::Sealed for ProgressSet {}

impl IntoCmdOptValue<Progress> for ProgressSet {
    fn into_value(self) -> Option<Progress> {
        Some(self.0)
    }
}

// ANSI control characters type
#[derive(Debug, Clone, Copy, Default)]
pub enum Ansi {
    #[default]
    Auto,
    Always,
    Never,
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        };
        write!(f, "{s}")
    }
}

// ANSI control characters markers
#[allow(private_bounds)]
pub trait AnsiOpt: IntoCmdOptValue<Ansi> + _priv::Sealed {}

pub struct AnsiNotSet;

impl AnsiOpt for AnsiNotSet {}
impl _priv::Sealed for AnsiNotSet {}

impl IntoCmdOptValue<Ansi> for AnsiNotSet {
    fn into_value(self) -> Option<Ansi> {
        None
    }
}

pub struct AnsiSet(Ansi);

impl AnsiOpt for AnsiSet {}
impl _priv::Sealed for AnsiSet {}

impl IntoCmdOptValue<Ansi> for AnsiSet {
    fn into_value(self) -> Option<Ansi> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
