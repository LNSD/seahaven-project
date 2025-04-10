use self::{info::DockerSystemInfoCmd, prune::DockerSystemPruneCmd};
use super::common::IntoCommand;

pub struct DockerSystemCmd(tokio::process::Command);

impl DockerSystemCmd {
    /// Create a new `docker system` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        let mut cmd = cmd.into_command();

        // Add the `system` subcommand
        cmd.arg("system");

        Self(cmd)
    }

    /// Create a new `docker system info` command
    pub fn info(self) -> DockerSystemInfoCmd {
        DockerSystemInfoCmd::new(self.0)
    }

    /// Create a new `docker system prune` command
    pub fn prune(self) -> DockerSystemPruneCmd {
        DockerSystemPruneCmd::new(self.0)
    }
}

impl IntoCommand for DockerSystemCmd {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub mod info {
    use std::marker::PhantomData;

    use super::IntoCommand;

    pub struct DockerSystemInfoCmd<F = DefaultFormat> {
        cmd: tokio::process::Command,
        _format: PhantomData<F>,
    }

    impl<F> DockerSystemInfoCmd<F> {
        /// Create a new `docker system info` command
        pub(crate) fn new(cmd: impl IntoCommand) -> Self {
            let mut cmd = cmd.into_command();

            // Add the `info` subcommand
            cmd.arg("info");

            Self {
                cmd,
                _format: PhantomData,
            }
        }
    }

    impl<F> IntoCommand for DockerSystemInfoCmd<F> {
        fn into_command(self) -> tokio::process::Command {
            self.cmd
        }
    }

    impl DockerSystemInfoCmd<DefaultFormat> {
        /// Create a new `docker system info` command with the `json` format.
        ///
        /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_info/#options)
        /// for more information.
        pub fn with_json_format(self) -> DockerSystemInfoCmd<WithJsonFormat> {
            let mut cmd = self.cmd;
            cmd.arg("--format");
            cmd.arg("json");
            DockerSystemInfoCmd::<WithJsonFormat> {
                cmd,
                _format: std::marker::PhantomData,
            }
        }

        /// Create a new `docker system info` command with a custom template format.
        ///
        /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_info/#options)
        /// for more information.
        pub fn with_custom_format(self, format: &str) -> DockerSystemInfoCmd<WithCustomFormat> {
            let mut cmd = self.cmd;
            cmd.arg("--format");
            cmd.arg(format);
            DockerSystemInfoCmd::<WithCustomFormat> {
                cmd,
                _format: std::marker::PhantomData,
            }
        }
    }

    /// A trait that represents a format option for the `docker system info` command.
    pub trait FormatOpt: _priv::Sealed {}

    pub struct DefaultFormat;

    impl FormatOpt for DefaultFormat {}
    impl _priv::Sealed for DefaultFormat {}

    pub struct WithJsonFormat;

    impl FormatOpt for WithJsonFormat {}
    impl _priv::Sealed for WithJsonFormat {}

    pub struct WithCustomFormat;

    impl FormatOpt for WithCustomFormat {}
    impl _priv::Sealed for WithCustomFormat {}

    #[allow(dead_code)]
    mod _priv {
        pub trait Sealed {}
    }
}

pub mod prune {
    use std::marker::PhantomData;

    use super::IntoCommand;

    pub struct DockerSystemPruneCmd<V = NoVolumes, A = NotAll, F = NoForce> {
        cmd: tokio::process::Command,
        _phantom: PhantomData<(V, A, F)>,
    }

    impl<V, A, F> DockerSystemPruneCmd<V, A, F>
    where
        V: VolumesOpt,
        A: AllOpt,
        F: ForceOpt,
    {
        pub(crate) fn new(cmd: impl IntoCommand) -> Self {
            let mut cmd = cmd.into_command();

            // Add the `prune` subcommand
            cmd.arg("prune");

            Self {
                cmd,
                _phantom: PhantomData,
            }
        }
    }

    impl<V, A, F> IntoCommand for DockerSystemPruneCmd<V, A, F>
    where
        V: VolumesOpt,
        A: AllOpt,
        F: ForceOpt,
    {
        fn into_command(self) -> tokio::process::Command {
            self.cmd
        }
    }

    impl<A, F> DockerSystemPruneCmd<NoVolumes, A, F>
    where
        A: AllOpt,
        F: ForceOpt,
    {
        /// Add the `--volumes` flag to the `docker system prune` command.
        ///
        /// This will remove all volumes not used by at least one container.
        ///
        /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_prune/#options)
        /// for more information.
        pub fn with_volumes(self) -> DockerSystemPruneCmd<Volumes, A, F> {
            let mut cmd = self.cmd;
            cmd.arg("--volumes");
            DockerSystemPruneCmd::<Volumes, A, F> {
                cmd,
                _phantom: PhantomData,
            }
        }
    }

    impl<V, F> DockerSystemPruneCmd<V, NotAll, F>
    where
        V: VolumesOpt,
        F: ForceOpt,
    {
        /// Removes all unused images, not just dangling ones.
        ///
        /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_prune/#options)
        /// for more information.
        pub fn with_all(self) -> DockerSystemPruneCmd<V, All, F> {
            let mut cmd = self.cmd;
            cmd.arg("--all");
            DockerSystemPruneCmd::<V, All, F> {
                cmd,
                _phantom: PhantomData,
            }
        }
    }

    impl<V, A> DockerSystemPruneCmd<V, A, NoForce>
    where
        V: VolumesOpt,
        A: AllOpt,
    {
        /// Add the `--force` flag to the `docker system prune` command.
        ///
        /// This will bypass the confirmation prompt.
        ///
        /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_prune/#options)
        /// for more information.
        pub fn with_force(self) -> DockerSystemPruneCmd<V, A, Force> {
            let mut cmd = self.cmd;
            cmd.arg("--force");
            DockerSystemPruneCmd::<V, A, Force> {
                cmd,
                _phantom: PhantomData,
            }
        }
    }

    /// A trait that represents the volume option for the `docker system prune` command.
    pub trait VolumesOpt: _priv::Sealed {}

    pub struct NoVolumes;

    impl VolumesOpt for NoVolumes {}
    impl _priv::Sealed for NoVolumes {}

    pub struct Volumes;

    impl VolumesOpt for Volumes {}
    impl _priv::Sealed for Volumes {}

    /// A trait that represents the force option for the `docker system prune` command.
    pub trait ForceOpt: _priv::Sealed {}

    pub struct NoForce;

    impl ForceOpt for NoForce {}
    impl _priv::Sealed for NoForce {}

    pub struct Force;

    impl ForceOpt for Force {}
    impl _priv::Sealed for Force {}

    /// A trait that represents the all option for the `docker system prune` command.
    pub trait AllOpt: _priv::Sealed {}

    pub struct NotAll;

    impl AllOpt for NotAll {}
    impl _priv::Sealed for NotAll {}

    pub struct All;

    impl AllOpt for All {}
    impl _priv::Sealed for All {}

    #[allow(dead_code)]
    mod _priv {
        pub trait Sealed {}
    }
}
