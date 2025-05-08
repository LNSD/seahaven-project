use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposeStopCmd<DR = DryRunNotSet, TO = TimeoutNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    dry_run_opt: DR,
    timeout_opt: TO,
    services_opt: S,
}

impl DockerComposeStopCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposeStopCmd {
        DockerComposeStopCmd {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            timeout_opt: TimeoutNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<DR, TO, S> IntoCommand for DockerComposeStopCmd<DR, TO, S>
where
    DR: DryRunOpt,
    TO: TimeoutOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `stop` subcommand
        cmd.arg("stop");

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // --timeout
        if let Some(timeout) = self.timeout_opt.into_value() {
            cmd.arg("--timeout").arg(timeout.to_string());
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<TO, S> DockerComposeStopCmd<DryRunNotSet, TO, S>
where
    TO: TimeoutOpt,
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/stop/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposeStopCmd<DryRunSet, TO, S> {
        DockerComposeStopCmd {
            cmd: self.cmd,
            dry_run_opt: DryRunSet(dry_run),
            timeout_opt: self.timeout_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, S> DockerComposeStopCmd<DR, TimeoutNotSet, S>
where
    DR: DryRunOpt,
    S: ServicesOpt,
{
    /// Specify a shutdown timeout in seconds with the `--timeout` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/stop/)
    /// for more information.
    pub fn with_timeout(
        self,
        timeout: impl Into<Option<u32>>,
    ) -> DockerComposeStopCmd<DR, TimeoutSet, S> {
        DockerComposeStopCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            timeout_opt: TimeoutSet(timeout.into()),
            services_opt: self.services_opt,
        }
    }
}

impl<DR, TO> DockerComposeStopCmd<DR, TO, ServicesNotSet>
where
    DR: DryRunOpt,
    TO: TimeoutOpt,
{
    /// Specify one or more services to stop.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/stop/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposeStopCmd<DR, TO, ServicesSet>
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

        DockerComposeStopCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            timeout_opt: self.timeout_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to stop.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/stop/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeStopCmd<DR, TO, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the dry run option for the `docker compose stop` command.
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

/// A trait that represents the timeout option for the `docker compose stop` command.
#[allow(private_bounds)]
pub trait TimeoutOpt: IntoCmdOptValue<u32> + _priv::Sealed {}

pub struct TimeoutNotSet;

impl TimeoutOpt for TimeoutNotSet {}
impl _priv::Sealed for TimeoutNotSet {}

impl IntoCmdOptValue<u32> for TimeoutNotSet {
    fn into_value(self) -> Option<u32> {
        None
    }
}

pub struct TimeoutSet(Option<u32>);

impl TimeoutOpt for TimeoutSet {}
impl _priv::Sealed for TimeoutSet {}

impl IntoCmdOptValue<u32> for TimeoutSet {
    fn into_value(self) -> Option<u32> {
        self.0
    }
}

/// A trait that represents whether services have been specified for the `docker compose stop` command.
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
