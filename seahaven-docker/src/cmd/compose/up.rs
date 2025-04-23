use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposeUpCmd<
    D = DetachNotSet,
    B = BuildNotSet,
    DR = DryRunNotSet,
    S = ServicesNotSet,
> {
    cmd: tokio::process::Command,
    detach_opt: D,
    build_opt: B,
    dry_run_opt: DR,
    services_opt: S,
}

impl DockerComposeUpCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposeUpCmd {
        DockerComposeUpCmd {
            cmd: cmd.into_command(),
            detach_opt: DetachNotSet,
            build_opt: BuildNotSet,
            dry_run_opt: DryRunNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<D, B, DR, S> IntoCommand for DockerComposeUpCmd<D, B, DR, S>
where
    D: DetachOpt,
    B: BuildOpt,
    DR: DryRunOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `up` subcommand
        cmd.arg("up");

        // --build
        if self.build_opt.into_flag_value() {
            cmd.arg("--build");
        }

        // --detach
        if self.detach_opt.into_flag_value() {
            cmd.arg("--detach");
        }

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<B, DR, S> DockerComposeUpCmd<DetachNotSet, B, DR, S>
where
    B: BuildOpt,
    DR: DryRunOpt,
    S: ServicesOpt,
{
    /// Run containers in the background with the `--detach` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_detach(self, detach: bool) -> DockerComposeUpCmd<DetachSet, B, DR, S> {
        DockerComposeUpCmd {
            cmd: self.cmd,
            detach_opt: DetachSet(detach),
            build_opt: self.build_opt,
            dry_run_opt: self.dry_run_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<D, DR, S> DockerComposeUpCmd<D, BuildNotSet, DR, S>
where
    D: DetachOpt,
    DR: DryRunOpt,
    S: ServicesOpt,
{
    /// Build images before starting containers with the `--build` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_build(self, build: bool) -> DockerComposeUpCmd<D, BuildSet, DR, S> {
        DockerComposeUpCmd {
            cmd: self.cmd,
            detach_opt: self.detach_opt,
            build_opt: BuildSet(build),
            dry_run_opt: self.dry_run_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<D, B, S> DockerComposeUpCmd<D, B, DryRunNotSet, S>
where
    D: DetachOpt,
    B: BuildOpt,
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposeUpCmd<D, B, DryRunSet, S> {
        DockerComposeUpCmd {
            cmd: self.cmd,
            detach_opt: self.detach_opt,
            build_opt: self.build_opt,
            dry_run_opt: DryRunSet(dry_run),
            services_opt: self.services_opt,
        }
    }
}

impl<D, B, DR> DockerComposeUpCmd<D, B, DR, ServicesNotSet>
where
    D: DetachOpt,
    B: BuildOpt,
    DR: DryRunOpt,
{
    /// Specify one or more services to start.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposeUpCmd<D, B, DR, ServicesSet>
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
            detach_opt: self.detach_opt,
            build_opt: self.build_opt,
            dry_run_opt: self.dry_run_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to include.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/up/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeUpCmd<D, B, DR, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the detach option for the `docker compose up` command.
#[allow(private_bounds)]
pub trait DetachOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct DetachNotSet;

impl DetachOpt for DetachNotSet {}
impl _priv::Sealed for DetachNotSet {}

impl IntoCmdOptValue<bool> for DetachNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct DetachSet(bool);

impl DetachOpt for DetachSet {}
impl _priv::Sealed for DetachSet {}

impl IntoCmdOptValue<bool> for DetachSet {
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

/// A trait that represents the dry run option for the `docker compose up` command.
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
