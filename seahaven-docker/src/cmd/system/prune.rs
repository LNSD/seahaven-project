use crate::cmd::common::{IntoCmdFlagValue, IntoCmdOptValue, IntoCommand};

pub struct DockerSystemPruneCmd<V = VolumesNotSet, A = AllNotSet, F = ForceNotSet> {
    cmd: tokio::process::Command,
    volumes_opt: V,
    all_opt: A,
    force_opt: F,
}

impl DockerSystemPruneCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            volumes_opt: VolumesNotSet,
            all_opt: AllNotSet,
            force_opt: ForceNotSet,
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
        let mut cmd = self.cmd;

        // Add the `prune` subcommand
        cmd.arg("prune");

        // --volumes
        if self.volumes_opt.into_flag_value() {
            cmd.arg("--volumes");
        }

        // --all
        if self.all_opt.into_flag_value() {
            cmd.arg("--all");
        }

        // --force
        if self.force_opt.into_flag_value() {
            cmd.arg("--force");
        }

        cmd
    }
}

impl<A, F> DockerSystemPruneCmd<VolumesNotSet, A, F>
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
    pub fn with_volumes(self, volumes: bool) -> DockerSystemPruneCmd<VolumesSet, A, F> {
        DockerSystemPruneCmd {
            cmd: self.cmd,
            volumes_opt: VolumesSet(volumes),
            all_opt: self.all_opt,
            force_opt: self.force_opt,
        }
    }
}

impl<V, F> DockerSystemPruneCmd<V, AllNotSet, F>
where
    V: VolumesOpt,
    F: ForceOpt,
{
    /// Removes all unused images, not just dangling ones.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_prune/#options)
    /// for more information.
    pub fn with_all(self, all: bool) -> DockerSystemPruneCmd<V, AllSet, F> {
        DockerSystemPruneCmd {
            cmd: self.cmd,
            volumes_opt: self.volumes_opt,
            all_opt: AllSet(all),
            force_opt: self.force_opt,
        }
    }
}

impl<V, A> DockerSystemPruneCmd<V, A, ForceNotSet>
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
    pub fn with_force(self, force: bool) -> DockerSystemPruneCmd<V, A, ForceSet> {
        DockerSystemPruneCmd {
            cmd: self.cmd,
            volumes_opt: self.volumes_opt,
            all_opt: self.all_opt,
            force_opt: ForceSet(force),
        }
    }
}

/// A trait that represents the volume option for the `docker system prune` command.
#[allow(private_bounds)]
pub trait VolumesOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct VolumesNotSet;

impl VolumesOpt for VolumesNotSet {}
impl _priv::Sealed for VolumesNotSet {}

impl IntoCmdOptValue<bool> for VolumesNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct VolumesSet(bool);

impl VolumesOpt for VolumesSet {}
impl _priv::Sealed for VolumesSet {}

impl IntoCmdOptValue<bool> for VolumesSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the force option for the `docker system prune` command.
#[allow(private_bounds)]
pub trait ForceOpt: IntoCmdOptValue<bool> + _priv::Sealed {}

pub struct ForceNotSet;

impl ForceOpt for ForceNotSet {}
impl _priv::Sealed for ForceNotSet {}

impl IntoCmdOptValue<bool> for ForceNotSet {
    fn into_value(self) -> Option<bool> {
        None
    }
}

pub struct ForceSet(bool);

impl ForceOpt for ForceSet {}
impl _priv::Sealed for ForceSet {}

impl IntoCmdOptValue<bool> for ForceSet {
    fn into_value(self) -> Option<bool> {
        Some(self.0)
    }
}

/// A trait that represents the all option for the `docker system prune` command.
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

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
