use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposePsCmd<DR = DryRunNotSet, A = AllNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    dry_run_opt: DR,
    all_opt: A,
    services_opt: S,
}

impl DockerComposePsCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposePsCmd {
        DockerComposePsCmd {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            all_opt: AllNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<DR, A, S> IntoCommand for DockerComposePsCmd<DR, A, S>
where
    DR: DryRunOpt,
    A: AllOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `ps` subcommand
        cmd.arg("ps");

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // --all
        if self.all_opt.into_flag_value() {
            cmd.arg("--all");
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<A, S> DockerComposePsCmd<DryRunNotSet, A, S>
where
    A: AllOpt,
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/ps/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposePsCmd<DryRunSet, A, S> {
        DockerComposePsCmd {
            cmd: self.cmd,
            dry_run_opt: DryRunSet(dry_run),
            all_opt: self.all_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, S> DockerComposePsCmd<DR, AllNotSet, S>
where
    DR: DryRunOpt,
    S: ServicesOpt,
{
    /// Show all stopped containers with the `--all` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/ps/)
    /// for more information.
    pub fn with_all(self, all: bool) -> DockerComposePsCmd<DR, AllSet, S> {
        DockerComposePsCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            all_opt: AllSet(all),
            services_opt: self.services_opt,
        }
    }
}

impl<DR, A> DockerComposePsCmd<DR, A, ServicesNotSet>
where
    DR: DryRunOpt,
    A: AllOpt,
{
    /// Specify one or more services to list.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/ps/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposePsCmd<DR, A, ServicesSet>
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

        DockerComposePsCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            all_opt: self.all_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to list.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/ps/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposePsCmd<DR, A, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the dry run option for the `docker compose ps` command.
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

/// A trait that represents the all option for the `docker compose ps` command.
#[allow(private_bounds)]
pub trait AllOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct AllNotSet;

impl AllOpt for AllNotSet {}
impl _priv::Sealed for AllNotSet {}

impl IntoCmdOptValue<bool> for AllNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct AllSet(bool);

impl AllOpt for AllSet {}
impl _priv::Sealed for AllSet {}

impl IntoCmdOptValue<bool> for AllSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents whether services have been specified for the `docker compose ps` command.
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
