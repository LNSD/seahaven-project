use super::{IntoCmdOptValue, IntoCommand};

pub struct DockerComposeDownCmd<V = VolumesNotSet> {
    cmd: tokio::process::Command,
    volumes_opt: V,
}

impl DockerComposeDownCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            volumes_opt: VolumesNotSet,
        }
    }
}

impl<V> IntoCommand for DockerComposeDownCmd<V>
where
    V: VolumesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `down` subcommand
        cmd.arg("down");

        // --volumes
        if matches!(self.volumes_opt.into_value(), Some(true)) {
            cmd.arg("--volumes");
        }

        cmd
    }
}

impl DockerComposeDownCmd<VolumesNotSet> {
    /// Remove named volumes declared in the `volumes` section of the Compose file
    /// and anonymous volumes attached to containers.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/down/)
    /// for more information.
    pub fn with_volumes(self, volumes: bool) -> DockerComposeDownCmd<VolumesSet> {
        DockerComposeDownCmd {
            cmd: self.cmd,
            volumes_opt: VolumesSet(volumes),
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

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
