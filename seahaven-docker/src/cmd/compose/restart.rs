use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposeRestartCmd<
    DR = DryRunNotSet,
    ND = NoDepsNotSet,
    TO = TimeoutNotSet,
    S = ServicesNotSet,
> {
    cmd: tokio::process::Command,
    dry_run_opt: DR,
    no_deps_opt: ND,
    timeout_opt: TO,
    services_opt: S,
}

impl DockerComposeRestartCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposeRestartCmd {
        DockerComposeRestartCmd {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            no_deps_opt: NoDepsNotSet,
            timeout_opt: TimeoutNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<DR, ND, TO, S> IntoCommand for DockerComposeRestartCmd<DR, ND, TO, S>
where
    DR: DryRunOpt,
    ND: NoDepsOpt,
    TO: TimeoutOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `restart` subcommand
        cmd.arg("restart");

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // --no-deps
        if self.no_deps_opt.into_flag_value() {
            cmd.arg("--no-deps");
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

impl<ND, TO, S> DockerComposeRestartCmd<DryRunNotSet, ND, TO, S>
where
    ND: NoDepsOpt,
    TO: TimeoutOpt,
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposeRestartCmd<DryRunSet, ND, TO, S> {
        DockerComposeRestartCmd {
            cmd: self.cmd,
            dry_run_opt: DryRunSet(dry_run),
            no_deps_opt: self.no_deps_opt,
            timeout_opt: self.timeout_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, TO, S> DockerComposeRestartCmd<DR, NoDepsNotSet, TO, S>
where
    DR: DryRunOpt,
    TO: TimeoutOpt,
    S: ServicesOpt,
{
    /// Don't restart dependent services with the `--no-deps` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn with_no_deps(self, no_deps: bool) -> DockerComposeRestartCmd<DR, NoDepsSet, TO, S> {
        DockerComposeRestartCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            no_deps_opt: NoDepsSet(no_deps),
            timeout_opt: self.timeout_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, ND, S> DockerComposeRestartCmd<DR, ND, TimeoutNotSet, S>
where
    DR: DryRunOpt,
    ND: NoDepsOpt,
    S: ServicesOpt,
{
    /// Specify a shutdown timeout in seconds with the `--timeout` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn with_timeout(self, timeout: u32) -> DockerComposeRestartCmd<DR, ND, TimeoutSet, S> {
        DockerComposeRestartCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            no_deps_opt: self.no_deps_opt,
            timeout_opt: TimeoutSet(timeout),
            services_opt: self.services_opt,
        }
    }
}

impl<DR, ND, TO> DockerComposeRestartCmd<DR, ND, TO, ServicesNotSet>
where
    DR: DryRunOpt,
    ND: NoDepsOpt,
    TO: TimeoutOpt,
{
    /// Specify one or more services to restart.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn with_services<I, T>(
        self,
        services: I,
    ) -> DockerComposeRestartCmd<DR, ND, TO, ServicesSet>
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

        DockerComposeRestartCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            no_deps_opt: self.no_deps_opt,
            timeout_opt: self.timeout_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to restart.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/restart/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeRestartCmd<DR, ND, TO, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the dry run option for the `docker compose restart` command.
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

/// A trait that represents the no deps option for the `docker compose restart` command.
#[allow(private_bounds)]
pub trait NoDepsOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct NoDepsNotSet;

impl NoDepsOpt for NoDepsNotSet {}
impl _priv::Sealed for NoDepsNotSet {}

impl IntoCmdOptValue<bool> for NoDepsNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct NoDepsSet(bool);

impl NoDepsOpt for NoDepsSet {}
impl _priv::Sealed for NoDepsSet {}

impl IntoCmdOptValue<bool> for NoDepsSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the timeout option for the `docker compose restart` command.
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

pub struct TimeoutSet(u32);

impl TimeoutOpt for TimeoutSet {}
impl _priv::Sealed for TimeoutSet {}

impl IntoCmdOptValue<u32> for TimeoutSet {
    fn into_value(self) -> Option<u32> {
        Some(self.0)
    }
}

/// A trait that represents whether services have been specified for the `docker compose restart` command.
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
