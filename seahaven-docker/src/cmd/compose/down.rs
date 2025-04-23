use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerComposeDownCmd<V = VolumesNotSet, DR = DryRunNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    volumes_opt: V,
    dry_run_opt: DR,
    services_opt: S,
}

impl DockerComposeDownCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            volumes_opt: VolumesNotSet,
            dry_run_opt: DryRunNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<V, DR, S> IntoCommand for DockerComposeDownCmd<V, DR, S>
where
    V: VolumesOpt,
    DR: DryRunOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `down` subcommand
        cmd.arg("down");

        // --volumes
        if self.volumes_opt.into_flag_value() {
            cmd.arg("--volumes");
        }

        // --dry-run
        if self.dry_run_opt.into_flag_value() {
            cmd.arg("--dry-run");
        }

        // Add services if specified
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }

        cmd
    }
}

impl<DR, S> DockerComposeDownCmd<VolumesNotSet, DR, S>
where
    DR: DryRunOpt,
    S: ServicesOpt,
{
    /// Remove named volumes declared in the `volumes` section of the Compose file
    /// and anonymous volumes attached to containers.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn with_volumes(self, volumes: bool) -> DockerComposeDownCmd<VolumesSet, DR, S> {
        DockerComposeDownCmd {
            cmd: self.cmd,
            volumes_opt: VolumesSet(volumes),
            dry_run_opt: self.dry_run_opt,
            services_opt: self.services_opt,
        }
    }
}

impl<V, S> DockerComposeDownCmd<V, DryRunNotSet, S>
where
    V: VolumesOpt,
    S: ServicesOpt,
{
    /// Run the command in dry run mode with the `--dry-run` flag.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn with_dry_run(self, dry_run: bool) -> DockerComposeDownCmd<V, DryRunSet, S> {
        DockerComposeDownCmd {
            cmd: self.cmd,
            volumes_opt: self.volumes_opt,
            dry_run_opt: DryRunSet(dry_run),
            services_opt: self.services_opt,
        }
    }
}

impl<V, DR> DockerComposeDownCmd<V, DR, ServicesNotSet>
where
    V: VolumesOpt,
    DR: DryRunOpt,
{
    /// Specify the services to stop and remove.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn with_services<I>(self, services: I) -> DockerComposeDownCmd<V, DR, ServicesSet>
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        DockerComposeDownCmd {
            cmd: self.cmd,
            volumes_opt: self.volumes_opt,
            dry_run_opt: self.dry_run_opt,
            services_opt: ServicesSet(services.into_iter().map(Into::into).collect()),
        }
    }
}

/// A trait that represents the volumes option for the `docker compose down` command.
#[allow(private_bounds)]
pub trait VolumesOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

/// Marker type for no volumes option specified.
pub struct VolumesNotSet;

impl VolumesOpt for VolumesNotSet {}
impl _priv::Sealed for VolumesNotSet {}

impl IntoCmdOptValue<bool> for VolumesNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

/// Marker type for volumes option specified.
pub struct VolumesSet(bool);

impl VolumesOpt for VolumesSet {}
impl _priv::Sealed for VolumesSet {}

impl IntoCmdOptValue<bool> for VolumesSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the dry run option for the `docker compose down` command.
#[allow(private_bounds)]
pub trait DryRunOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

/// Marker type for no dry run option specified.
pub struct DryRunNotSet;

impl DryRunOpt for DryRunNotSet {}
impl _priv::Sealed for DryRunNotSet {}

impl IntoCmdOptValue<bool> for DryRunNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

/// Marker type for dry run option specified.
pub struct DryRunSet(bool);

impl DryRunOpt for DryRunSet {}
impl _priv::Sealed for DryRunSet {}

impl IntoCmdOptValue<bool> for DryRunSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the services option for the `docker compose down` command.
#[allow(private_bounds)]
pub trait ServicesOpt: IntoCmdOptValue<Vec<String>> + _priv::Sealed {}

/// Marker type for no services option specified.
pub struct ServicesNotSet;

impl ServicesOpt for ServicesNotSet {}
impl _priv::Sealed for ServicesNotSet {}

impl IntoCmdOptValue<Vec<String>> for ServicesNotSet {
    fn into_value(self) -> Option<Vec<String>> {
        None
    }
}

/// Marker type for services option specified.
pub struct ServicesSet(Vec<String>);

impl ServicesOpt for ServicesSet {}
impl _priv::Sealed for ServicesSet {}

impl IntoCmdOptValue<Vec<String>> for ServicesSet {
    fn into_value(self) -> Option<Vec<String>> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
