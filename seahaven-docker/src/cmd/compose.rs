use std::marker::PhantomData;

use self::{
    build::DockerComposeBuildCmd,
    down::DockerComposeDownCmd,
    opts::{
        AnsiAlways, AnsiAuto, AnsiNever, NoProjectName, ProgressAuto, ProgressJson, ProgressPlain,
        ProgressQuiet, ProgressTty, WithProjectName,
    },
    pull::DockerComposePullCmd,
    up::DockerComposeUpCmd,
};
use super::common::IntoCommand;

pub struct DockerComposeCmd<N = NoProjectName, P = ProgressAuto, A = AnsiAuto>(
    tokio::process::Command,
    PhantomData<(N, P, A)>,
);

impl<N, P, A> DockerComposeCmd<N, P, A> {
    pub(crate) fn new(mut cmd: tokio::process::Command) -> Self {
        cmd.arg("compose");
        Self(cmd, PhantomData)
    }

    /// Pull the images for the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose pull`](https://docs.docker.com/compose/reference/pull/)
    /// for more information.
    pub fn pull(self) -> DockerComposePullCmd {
        DockerComposePullCmd::new(self.0)
    }

    /// Build the images for the services defined in the `docker-compose.yml` file.
    ///
    /// See [`Docker Compose Build`](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn build(self) -> DockerComposeBuildCmd {
        DockerComposeBuildCmd::new(self.0)
    }

    /// Start the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose up`](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn up(self) -> DockerComposeUpCmd {
        DockerComposeUpCmd::new(self.0)
    }

    /// Stop the services defined in the `docker-compose.yml` file.
    ///
    /// See [`docker compose down`](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn down(self) -> DockerComposeDownCmd {
        DockerComposeDownCmd::new(self.0)
    }

    /// Specify an alternate compose file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--file` option.
    pub fn with_file<S>(mut self, file: S) -> Self
    where
        S: AsRef<str>,
    {
        self.0.arg("--file").arg(file.as_ref());
        self
    }

    /// Specify multiple compose files.
    ///
    /// Files will be applied in the order they're specified, with later files overriding
    /// and adding to their predecessors.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--file` option.
    pub fn with_files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for file in files {
            self.0.arg("--file").arg(file.as_ref());
        }
        self
    }

    /// Specify an alternate environment file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/)
    /// for more information about the `--env-file` option.
    pub fn with_env_file<S>(mut self, env_file: S) -> Self
    where
        S: AsRef<str>,
    {
        self.0.arg("--env-file").arg(env_file.as_ref());
        self
    }
}

impl<N, P> DockerComposeCmd<N, P, AnsiAuto> {
    /// Always print ANSI control characters.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/)
    /// for more information about the `--ansi` option.
    pub fn with_ansi_always(mut self) -> DockerComposeCmd<N, P, AnsiAlways> {
        self.0.arg("--ansi").arg("always");
        DockerComposeCmd(self.0, PhantomData)
    }

    /// Never print ANSI control characters.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/)
    /// for more information about the `--ansi` option.
    pub fn with_ansi_never(mut self) -> DockerComposeCmd<N, P, AnsiNever> {
        self.0.arg("--ansi").arg("never");
        DockerComposeCmd(self.0, PhantomData)
    }
}

impl<P, A> DockerComposeCmd<NoProjectName, P, A> {
    /// Specify a project name for the Docker Compose deployment.
    ///
    /// The project name is used as a prefix for container names and creates an isolated
    /// environment for the services. By default, Docker Compose uses the directory name
    /// of the compose file.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/overview/#use--p-to-specify-a-project-name)
    /// for more information about the `--project-name` option.
    pub fn with_project_name<S>(mut self, name: S) -> DockerComposeCmd<WithProjectName, P, A>
    where
        S: AsRef<str>,
    {
        self.0.arg("--project-name").arg(name.as_ref());
        DockerComposeCmd(self.0, PhantomData)
    }
}

impl<N, A> DockerComposeCmd<N, ProgressAuto, A> {
    /// Set the type of progress output to TTY
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_tty_progress(mut self) -> DockerComposeCmd<N, ProgressTty, A> {
        self.0.arg("--progress").arg("tty");
        DockerComposeCmd(self.0, PhantomData)
    }

    /// Set the type of progress output to plain text
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_plain_progress(mut self) -> DockerComposeCmd<N, ProgressPlain, A> {
        self.0.arg("--progress").arg("plain");
        DockerComposeCmd(self.0, PhantomData)
    }

    /// Set the type of progress output to JSON
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_json_progress(mut self) -> DockerComposeCmd<N, ProgressJson, A> {
        self.0.arg("--progress").arg("json");
        DockerComposeCmd(self.0, PhantomData)
    }

    /// Set the type of progress output to quiet (no output)
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/#progress)
    /// for more information.
    pub fn with_quiet_progress(mut self) -> DockerComposeCmd<N, ProgressQuiet, A> {
        self.0.arg("--progress").arg("quiet");
        DockerComposeCmd(self.0, PhantomData)
    }
}

impl<N, P, A> IntoCommand for DockerComposeCmd<N, P, A> {
    fn into_command(self) -> tokio::process::Command {
        self.0
    }
}

pub mod opts {
    // Progress markers
    pub trait ProgressOpt: _priv::Sealed {}

    pub struct ProgressAuto;
    impl ProgressOpt for ProgressAuto {}
    impl _priv::Sealed for ProgressAuto {}

    pub struct ProgressTty;
    impl ProgressOpt for ProgressTty {}
    impl _priv::Sealed for ProgressTty {}

    pub struct ProgressPlain;
    impl ProgressOpt for ProgressPlain {}
    impl _priv::Sealed for ProgressPlain {}

    pub struct ProgressJson;
    impl ProgressOpt for ProgressJson {}
    impl _priv::Sealed for ProgressJson {}

    pub struct ProgressQuiet;
    impl ProgressOpt for ProgressQuiet {}
    impl _priv::Sealed for ProgressQuiet {}

    // Project name markers
    pub trait ProjectNameOpt: _priv::Sealed {}

    pub struct NoProjectName;
    impl ProjectNameOpt for NoProjectName {}
    impl _priv::Sealed for NoProjectName {}

    pub struct WithProjectName;
    impl ProjectNameOpt for WithProjectName {}
    impl _priv::Sealed for WithProjectName {}

    // ANSI control characters markers
    pub trait AnsiOpt: _priv::Sealed {}

    pub struct AnsiAuto;
    impl AnsiOpt for AnsiAuto {}
    impl _priv::Sealed for AnsiAuto {}

    pub struct AnsiAlways;
    impl AnsiOpt for AnsiAlways {}
    impl _priv::Sealed for AnsiAlways {}

    pub struct AnsiNever;
    impl AnsiOpt for AnsiNever {}
    impl _priv::Sealed for AnsiNever {}

    // Private module for sealing traits
    #[allow(dead_code)]
    mod _priv {
        pub trait Sealed {}
    }
}

pub mod build {
    use std::marker::PhantomData;

    use super::IntoCommand;

    pub struct DockerComposeBuildCmd<S = NoServices> {
        cmd: tokio::process::Command,
        _args: PhantomData<S>,
    }

    impl<S> DockerComposeBuildCmd<S>
    where
        S: ServicesOpt,
    {
        pub(crate) fn new(mut cmd: tokio::process::Command) -> Self {
            cmd.arg("build");
            Self {
                cmd,
                _args: PhantomData,
            }
        }
    }

    impl<S> IntoCommand for DockerComposeBuildCmd<S>
    where
        S: ServicesOpt,
    {
        fn into_command(self) -> tokio::process::Command {
            self.cmd
        }
    }

    impl DockerComposeBuildCmd<NoServices> {
        /// Specify which services to build.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
        /// for more information.
        pub fn with_services<I, S>(self, services: I) -> DockerComposeBuildCmd<WithServices>
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            let mut cmd = self.cmd;
            for service in services {
                cmd.arg(service.as_ref());
            }
            DockerComposeBuildCmd {
                cmd,
                _args: PhantomData,
            }
        }

        /// Specify a single service to build.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
        /// for more information.
        pub fn with_service<S>(self, service: S) -> DockerComposeBuildCmd<WithServices>
        where
            S: AsRef<str>,
        {
            self.with_services([service])
        }

        /// Add a build argument in the form of `<key>=<value>`.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
        /// for more information.
        pub fn with_build_arg<S>(self, arg: S) -> Self
        where
            S: AsRef<str>,
        {
            let mut cmd = self.cmd;
            cmd.arg("--build-arg").arg(arg.as_ref());
            DockerComposeBuildCmd {
                cmd,
                _args: PhantomData,
            }
        }

        /// Add multiple build arguments.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
        /// for more information.
        pub fn with_build_args<I, S>(self, args: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            let mut cmd = self.cmd;
            for arg in args {
                cmd.arg("--build-arg").arg(arg.as_ref());
            }
            DockerComposeBuildCmd {
                cmd,
                _args: PhantomData,
            }
        }
    }

    /// Marker trait for services options.
    pub trait ServicesOpt {}

    /// Marker type for no services specified.
    pub struct NoServices;
    impl ServicesOpt for NoServices {}

    /// Marker type for services specified.
    pub struct WithServices;
    impl ServicesOpt for WithServices {}
}

pub mod up {
    use std::marker::PhantomData;

    use super::IntoCommand;

    pub struct DockerComposeUpCmd<D = NotDetached, B = NoBuild, S = NoServices> {
        cmd: tokio::process::Command,
        _args: PhantomData<(D, B, S)>,
    }

    impl<D, B, S> DockerComposeUpCmd<D, B, S>
    where
        D: DetachedOpt,
        B: BuildOpt,
        S: ServicesOpt,
    {
        pub(crate) fn new(mut cmd: tokio::process::Command) -> Self {
            cmd.arg("up");
            Self {
                cmd,
                _args: PhantomData,
            }
        }
    }

    impl<D, B, S> IntoCommand for DockerComposeUpCmd<D, B, S>
    where
        D: DetachedOpt,
        B: BuildOpt,
        S: ServicesOpt,
    {
        fn into_command(self) -> tokio::process::Command {
            self.cmd
        }
    }

    impl<B> DockerComposeUpCmd<NotDetached, B, NoServices>
    where
        B: BuildOpt,
    {
        /// Run containers in the background with the `--detached` flag.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
        /// for more information.
        pub fn with_detached(self) -> DockerComposeUpCmd<Detached, B, NoServices> {
            let mut cmd = self.cmd;
            cmd.arg("--detached");
            DockerComposeUpCmd {
                cmd,
                _args: PhantomData,
            }
        }
    }

    impl<D> DockerComposeUpCmd<D, NoBuild, NoServices>
    where
        D: DetachedOpt,
    {
        /// Build images before starting containers with the `--build` flag.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
        /// for more information.
        pub fn with_build(self) -> DockerComposeUpCmd<D, WithBuild, NoServices> {
            let mut cmd = self.cmd;
            cmd.arg("--build");
            DockerComposeUpCmd {
                cmd,
                _args: PhantomData,
            }
        }
    }

    impl<D, B> DockerComposeUpCmd<D, B, NoServices>
    where
        D: DetachedOpt,
        B: BuildOpt,
    {
        /// Specify one or more services to start.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
        /// for more information.
        pub fn with_services<I, S>(self, services: I) -> DockerComposeUpCmd<D, B, WithServices>
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            let mut cmd = self.cmd;
            for service in services {
                cmd.arg(service.as_ref());
            }
            DockerComposeUpCmd {
                cmd,
                _args: PhantomData,
            }
        }

        /// Specify a single service to include.
        ///
        /// This is a convenience method for `with_services([service])`.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
        /// for more information.
        pub fn with_service<S>(self, service: S) -> DockerComposeUpCmd<D, B, WithServices>
        where
            S: AsRef<str>,
        {
            self.with_services([service])
        }
    }

    /// A trait that represents the detached option for the `docker compose up` command.
    pub trait DetachedOpt: _priv::Sealed {}

    pub struct NotDetached;

    impl DetachedOpt for NotDetached {}
    impl _priv::Sealed for NotDetached {}

    pub struct Detached;

    impl DetachedOpt for Detached {}
    impl _priv::Sealed for Detached {}

    /// A trait that represents the build option for the `docker compose up` command.
    pub trait BuildOpt: _priv::Sealed {}

    pub struct NoBuild;

    impl BuildOpt for NoBuild {}
    impl _priv::Sealed for NoBuild {}

    pub struct WithBuild;

    impl BuildOpt for WithBuild {}
    impl _priv::Sealed for WithBuild {}

    /// A trait that represents whether services have been specified for the `docker compose up` command.
    pub trait ServicesOpt: _priv::Sealed {}

    pub struct NoServices;

    impl ServicesOpt for NoServices {}
    impl _priv::Sealed for NoServices {}

    pub struct WithServices;

    impl ServicesOpt for WithServices {}
    impl _priv::Sealed for WithServices {}

    #[allow(dead_code)]
    mod _priv {
        pub trait Sealed {}
    }
}

pub mod down {
    use std::marker::PhantomData;

    use super::IntoCommand;

    pub struct DockerComposeDownCmd<V = NoVolumes> {
        cmd: tokio::process::Command,
        _args: PhantomData<V>,
    }

    impl<V> DockerComposeDownCmd<V>
    where
        V: VolumesOpt,
    {
        pub(crate) fn new(mut cmd: tokio::process::Command) -> Self {
            cmd.arg("down");
            Self {
                cmd,
                _args: PhantomData,
            }
        }
    }

    impl<V> IntoCommand for DockerComposeDownCmd<V>
    where
        V: VolumesOpt,
    {
        fn into_command(self) -> tokio::process::Command {
            self.cmd
        }
    }

    impl DockerComposeDownCmd<NoVolumes> {
        /// Remove named volumes declared in the `volumes` section of the Compose file
        /// and anonymous volumes attached to containers.
        ///
        /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/down/)
        /// for more information.
        pub fn with_volumes(self) -> DockerComposeDownCmd<WithVolumes> {
            let mut cmd = self.cmd;
            cmd.arg("--volumes");
            DockerComposeDownCmd {
                cmd,
                _args: PhantomData,
            }
        }
    }

    /// A trait that represents the volumes option for the `docker compose down` command.
    pub trait VolumesOpt: _priv::Sealed {}

    /// Marker type for no volumes option specified.
    pub struct NoVolumes;
    impl VolumesOpt for NoVolumes {}
    impl _priv::Sealed for NoVolumes {}

    /// Marker type for volumes option specified.
    pub struct WithVolumes;
    impl VolumesOpt for WithVolumes {}
    impl _priv::Sealed for WithVolumes {}

    #[allow(dead_code)]
    mod _priv {
        pub trait Sealed {}
    }
}

pub mod pull {
    use super::IntoCommand;

    pub struct DockerComposePullCmd(tokio::process::Command);

    impl DockerComposePullCmd {
        pub(crate) fn new(mut cmd: tokio::process::Command) -> Self {
            cmd.arg("pull");
            Self(cmd)
        }
    }

    impl IntoCommand for DockerComposePullCmd {
        fn into_command(self) -> tokio::process::Command {
            self.0
        }
    }
}
