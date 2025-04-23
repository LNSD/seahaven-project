use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposeLogsCmd<
    DR = DryRunNotSet,
    F = FollowNotSet,
    TS = TimestampsNotSet,
    S = ServicesNotSet,
> {
    cmd: tokio::process::Command,
    dry_run_opt: DR,
    follow_opt: F,
    timestamps_opt: TS,
    services_opt: S,
}

impl DockerComposeLogsCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> DockerComposeLogsCmd {
        DockerComposeLogsCmd {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            follow_opt: FollowNotSet,
            timestamps_opt: TimestampsNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<DR, F, TS, S> IntoCommand for DockerComposeLogsCmd<DR, F, TS, S>
where
    DR: DryRunOpt,
    F: FollowOpt,
    TS: TimestampsOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `logs` subcommand
        cmd.arg("logs");

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // --follow
        if self.follow_opt.into_flag_value() {
            cmd.arg("--follow");
        }

        // --timestamps
        if self.timestamps_opt.into_flag_value() {
            cmd.arg("--timestamps");
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<F, TS, S> DockerComposeLogsCmd<DryRunNotSet, F, TS, S> {
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposeLogsCmd<DryRunSet, F, TS, S> {
        DockerComposeLogsCmd {
            cmd: self.cmd,
            dry_run_opt: DryRunSet(dry_run),
            follow_opt: self.follow_opt,
            timestamps_opt: self.timestamps_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, TS, S> DockerComposeLogsCmd<DR, FollowNotSet, TS, S> {
    /// Follow log output with the `--follow` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn with_follow(self, follow: bool) -> DockerComposeLogsCmd<DR, FollowSet, TS, S> {
        DockerComposeLogsCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            follow_opt: FollowSet(follow),
            timestamps_opt: self.timestamps_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<DR, F, S> DockerComposeLogsCmd<DR, F, TimestampsNotSet, S> {
    /// Show timestamps with the `--timestamps` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn with_timestamps(
        self,
        timestamps: bool,
    ) -> DockerComposeLogsCmd<DR, F, TimestampsSet, S> {
        DockerComposeLogsCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            follow_opt: self.follow_opt,
            timestamps_opt: TimestampsSet(timestamps),
            services_opt: self.services_opt,
        }
    }
}

impl<DR, F, TS> DockerComposeLogsCmd<DR, F, TS, ServicesNotSet> {
    /// Specify one or more services to show logs for.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposeLogsCmd<DR, F, TS, ServicesSet>
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

        DockerComposeLogsCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            follow_opt: self.follow_opt,
            timestamps_opt: self.timestamps_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to show logs for.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/logs/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeLogsCmd<DR, F, TS, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// A trait that represents the dry run option for the `docker compose logs` command.
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

/// A trait that represents the follow option for the `docker compose logs` command.
#[allow(private_bounds)]
pub trait FollowOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct FollowNotSet;

impl FollowOpt for FollowNotSet {}
impl _priv::Sealed for FollowNotSet {}

impl IntoCmdOptValue<bool> for FollowNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct FollowSet(bool);

impl FollowOpt for FollowSet {}
impl _priv::Sealed for FollowSet {}

impl IntoCmdOptValue<bool> for FollowSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the timestamps option for the `docker compose logs` command.
#[allow(private_bounds)]
pub trait TimestampsOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct TimestampsNotSet;

impl TimestampsOpt for TimestampsNotSet {}
impl _priv::Sealed for TimestampsNotSet {}

impl IntoCmdOptValue<bool> for TimestampsNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct TimestampsSet(bool);

impl TimestampsOpt for TimestampsSet {}
impl _priv::Sealed for TimestampsSet {}

impl IntoCmdOptValue<bool> for TimestampsSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents whether services have been specified for the `docker compose logs` command.
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
