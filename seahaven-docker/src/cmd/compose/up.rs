use super::{IntoCmdOptValue, IntoCommand};

pub struct DockerComposeUpCmd<D = DetachedNotSet, B = BuildNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    detached_opt: D,
    build_opt: B,
    services_opt: S,
}

impl DockerComposeUpCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposeUpCmd {
        DockerComposeUpCmd {
            cmd: cmd.into_command(),
            detached_opt: DetachedNotSet,
            build_opt: BuildNotSet,
            services_opt: ServicesNotSet,
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
        let mut cmd = self.cmd;

        // Add the `up` subcommand
        cmd.arg("up");

        // --build
        if matches!(self.build_opt.into_value(), Some(true)) {
            cmd.arg("--build");
        }

        // --detached
        if matches!(self.detached_opt.into_value(), Some(true)) {
            cmd.arg("--detached");
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<B, S> DockerComposeUpCmd<DetachedNotSet, B, S>
where
    B: BuildOpt,
    S: ServicesOpt,
{
    /// Run containers in the background with the `--detached` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_detached(self, detached: bool) -> DockerComposeUpCmd<DetachedSet, B, S> {
        DockerComposeUpCmd {
            cmd: self.cmd,
            detached_opt: DetachedSet(detached),
            build_opt: self.build_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<D, S> DockerComposeUpCmd<D, BuildNotSet, S>
where
    D: DetachedOpt,
    S: ServicesOpt,
{
    /// Build images before starting containers with the `--build` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_build(self, build: bool) -> DockerComposeUpCmd<D, BuildSet, S> {
        DockerComposeUpCmd {
            cmd: self.cmd,
            detached_opt: self.detached_opt,
            build_opt: BuildSet(build),
            services_opt: self.services_opt,
        }
    }
}

impl<D, B> DockerComposeUpCmd<D, B, ServicesNotSet>
where
    D: DetachedOpt,
    B: BuildOpt,
{
    /// Specify one or more services to start.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposeUpCmd<D, B, ServicesSet>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        let services = services
            .into_iter()
            .filter_map(|s| {
                // Filter out empty service names
                let service = s.as_ref();
                if service.is_empty() {
                    None
                } else {
                    Some(service.to_string())
                }
            })
            .collect();

        DockerComposeUpCmd {
            cmd: self.cmd,
            detached_opt: self.detached_opt,
            build_opt: self.build_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to include.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeUpCmd<D, B, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the detached option for the `docker compose up` command.
#[allow(private_bounds)]
pub trait DetachedOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct DetachedNotSet;

impl DetachedOpt for DetachedNotSet {}
impl _priv::Sealed for DetachedNotSet {}

impl IntoCmdOptValue<bool> for DetachedNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct DetachedSet(bool);

impl DetachedOpt for DetachedSet {}
impl _priv::Sealed for DetachedSet {}

impl IntoCmdOptValue<bool> for DetachedSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the build option for the `docker compose up` command.
#[allow(private_bounds)]
pub trait BuildOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct BuildNotSet;

impl BuildOpt for BuildNotSet {}
impl _priv::Sealed for BuildNotSet {}

impl IntoCmdOptValue<bool> for BuildNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct BuildSet(bool);

impl BuildOpt for BuildSet {}
impl _priv::Sealed for BuildSet {}

impl IntoCmdOptValue<bool> for BuildSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents whether services have been specified for the `docker compose up` command.
#[allow(private_bounds)]
pub trait ServicesOpt: IntoCmdOptValue<Box<[String]>> + _priv::Sealed {}

pub struct ServicesNotSet;

impl ServicesOpt for ServicesNotSet {}
impl _priv::Sealed for ServicesNotSet {}

impl IntoCmdOptValue<Box<[String]>> for ServicesNotSet {
    fn into_value(self) -> Option<Box<[String]>> {
        None
    }
}

pub struct ServicesSet(Box<[String]>);

impl ServicesOpt for ServicesSet {}
impl _priv::Sealed for ServicesSet {}

impl IntoCmdOptValue<Box<[String]>> for ServicesSet {
    fn into_value(self) -> Option<Box<[String]>> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
