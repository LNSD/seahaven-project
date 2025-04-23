use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposePullCmd<DR = DryRunNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    dry_run_opt: DR,
    services_opt: S,
}

impl DockerComposePullCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposePullCmd {
        DockerComposePullCmd {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<DR, S> IntoCommand for DockerComposePullCmd<DR, S>
where
    DR: DryRunOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `pull` subcommand
        cmd.arg("pull");

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

impl<S> DockerComposePullCmd<DryRunNotSet, S>
where
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/pull/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposePullCmd<DryRunSet, S> {
        DockerComposePullCmd {
            cmd: self.cmd,
            dry_run_opt: DryRunSet(dry_run),
            services_opt: self.services_opt,
        }
    }
}

impl<DR> DockerComposePullCmd<DR, ServicesNotSet>
where
    DR: DryRunOpt,
{
    /// Specify one or more services to pull.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/pull/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposePullCmd<DR, ServicesSet>
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

        DockerComposePullCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to pull.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/pull/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposePullCmd<DR, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the dry run option for the `docker compose pull` command.
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

/// A trait that represents whether services have been specified for the `docker compose pull` command.
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
