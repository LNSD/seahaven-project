use super::{IntoCmdOptValue, IntoCommand};

pub struct DockerSystemInfoCmd<F = FormatNotSet> {
    cmd: tokio::process::Command,
    format_opt: F,
}

impl DockerSystemInfoCmd {
    /// Create a new `docker system info` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            format_opt: FormatNotSet,
        }
    }
}

impl<F> IntoCommand for DockerSystemInfoCmd<F>
where
    F: FormatOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `info` subcommand
        cmd.arg("info");

        // --format <format>
        if let Some(format) = self.format_opt.into_value() {
            cmd.arg("--format").arg(format.to_string());
        }

        cmd
    }
}

impl DockerSystemInfoCmd<FormatNotSet> {
    /// Create a new `docker system info` command with the `json` format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_info/#options)
    /// for more information.
    pub fn with_json_format(self) -> DockerSystemInfoCmd<FormatSet> {
        DockerSystemInfoCmd {
            cmd: self.cmd,
            format_opt: FormatSet(Format::Json),
        }
    }

    /// Create a new `docker system info` command with a custom template format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/system_info/#options)
    /// for more information.
    pub fn with_custom_format(self, format: &str) -> DockerSystemInfoCmd<FormatSet> {
        DockerSystemInfoCmd {
            cmd: self.cmd,
            format_opt: FormatSet(Format::Custom(format.to_string())),
        }
    }
}

/// A trait that represents a format option for the `docker system info` command.
#[allow(private_bounds)]
pub trait FormatOpt: IntoCmdOptValue<Format> + _priv::Sealed {}

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
