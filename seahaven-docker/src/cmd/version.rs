use super::common::{IntoCmdOptValue, IntoCommand};

pub struct DockerVersionCmd<F = FormatNotSet> {
    cmd: tokio::process::Command,
    format_opt: F,
}

impl DockerVersionCmd {
    /// Create a new `docker version` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            format_opt: FormatNotSet,
        }
    }
}

impl<F> IntoCommand for DockerVersionCmd<F>
where
    F: FormatOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `version` subcommand
        cmd.arg("version");

        // --format <format>
        if let Some(format) = self.format_opt.into_value() {
            cmd.arg("--format").arg(format.to_string());
        }

        cmd
    }
}

impl DockerVersionCmd<FormatNotSet> {
    /// Create a new `docker version` command with the `json` format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/version/#options)
    /// for more information.
    pub fn with_json_format(self) -> DockerVersionCmd<FormatSet> {
        DockerVersionCmd {
            cmd: self.cmd,
            format_opt: FormatSet(Format::Json),
        }
    }

    /// Create a new `docker version` command with a custom template format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/version/#options)
    /// for more information.
    pub fn with_custom_format<T>(self, format: T) -> DockerVersionCmd<FormatSet>
    where
        T: Into<String>,
    {
        DockerVersionCmd {
            cmd: self.cmd,
            format_opt: FormatSet(Format::Custom(format.into())),
        }
    }
}

// The format option for the `docker version` command.
#[derive(Debug, Clone)]
pub enum Format {
    Json,
    Custom(String),
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Json => "json",
            Self::Custom(s) => s.as_str(),
        };

        write!(f, "{}", s)
    }
}

/// A trait that represents a format option for the `docker version` command.
#[allow(private_bounds)]
pub trait FormatOpt: IntoCmdOptValue<Format> + _priv::Sealed {}

pub struct FormatNotSet;

impl FormatOpt for FormatNotSet {}
impl _priv::Sealed for FormatNotSet {}

impl IntoCmdOptValue<Format> for FormatNotSet {
    fn into_value(self) -> Option<Format> {
        None
    }
}

pub struct FormatSet(Format);

impl FormatOpt for FormatSet {}
impl _priv::Sealed for FormatSet {}

impl IntoCmdOptValue<Format> for FormatSet {
    fn into_value(self) -> Option<Format> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
