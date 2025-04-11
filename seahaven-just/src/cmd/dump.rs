use super::common::{IntoCmdOptValue, IntoCommand};

pub struct JustDumpCmd<F = DumpFormatNotSet> {
    cmd: tokio::process::Command,
    format_opt: F,
}

impl JustDumpCmd {
    /// Create a new `just --dump` command
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            format_opt: DumpFormatNotSet,
        }
    }
}

impl<F> IntoCommand for JustDumpCmd<F>
where
    F: DumpFormatOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `dump` subcommand
        cmd.arg("--dump");

        // --format <format>
        if let Some(format) = self.format_opt.into_value() {
            cmd.arg("--format").arg(format.as_str());
        }

        cmd
    }
}

impl JustDumpCmd<DumpFormatNotSet> {
    /// Configures the command to use the "just" format for dumping.
    /// This format outputs the justfile in its original format.
    ///
    /// See the [Just documentation](https://github.com/casey/just) for more information.
    pub fn with_dump_just_format(self) -> JustDumpCmd<DumpFormatSet> {
        JustDumpCmd {
            cmd: self.cmd,
            format_opt: DumpFormatSet(DumpFormat::Just),
        }
    }

    /// Configures the command to use the "json" format for dumping.
    /// This format outputs the justfile as a JSON object.
    ///
    /// See the [Just documentation](https://github.com/casey/just) for more information.
    pub fn with_dump_json_format(self) -> JustDumpCmd<DumpFormatSet> {
        JustDumpCmd {
            cmd: self.cmd,
            format_opt: DumpFormatSet(DumpFormat::Json),
        }
    }
}

/// Available formats for the `just --dump` command output.
///
/// See the [Just documentation](https://github.com/casey/just#dump) for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DumpFormat {
    /// Output the justfile in its original format
    Just,
    /// Output the justfile as a JSON object
    Json,
}

impl DumpFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Just => "just",
            Self::Json => "json",
        }
    }
}

// Dump format option markers
#[allow(private_bounds)]
pub trait DumpFormatOpt: IntoCmdOptValue<DumpFormat> + _priv::Sealed {}

pub struct DumpFormatNotSet;

impl DumpFormatOpt for DumpFormatNotSet {}
impl _priv::Sealed for DumpFormatNotSet {}

impl IntoCmdOptValue<DumpFormat> for DumpFormatNotSet {
    fn into_value(self) -> Option<DumpFormat> {
        None
    }
}

pub struct DumpFormatSet(DumpFormat);

impl DumpFormatOpt for DumpFormatSet {}
impl _priv::Sealed for DumpFormatSet {}

impl IntoCmdOptValue<DumpFormat> for DumpFormatSet {
    fn into_value(self) -> Option<DumpFormat> {
        Some(self.0)
    }
}

#[allow(dead_code)]
mod _priv {
    pub trait Sealed {}
}
