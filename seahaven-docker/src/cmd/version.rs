use std::marker::PhantomData;

use super::common::IntoCommand;

pub struct DockerVersionCmd<F = NoFormatOpt> {
    cmd: tokio::process::Command,
    _format: PhantomData<F>,
}

impl<F> DockerVersionCmd<F> {
    /// Create a new `docker version` command
    pub(crate) fn new(cmd: tokio::process::Command) -> Self {
        Self {
            cmd,
            _format: PhantomData,
        }
    }
}

impl<F> IntoCommand for DockerVersionCmd<F> {
    fn into_command(self) -> tokio::process::Command {
        self.cmd
    }
}

impl DockerVersionCmd<NoFormatOpt> {
    /// Create a new `docker version` command with the `json` format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/version/#options)
    /// for more information.
    pub fn with_json_format(self) -> DockerVersionCmd<JsonFormatOpt> {
        let mut cmd = self.cmd;
        cmd.arg("--format");
        cmd.arg("json");
        DockerVersionCmd {
            cmd,
            _format: PhantomData,
        }
    }

    /// Create a new `docker version` command with a custom template format.
    ///
    /// See the [Docker CLI documentation](https://docs.docker.com/engine/reference/commandline/version/#options)
    /// for more information.
    pub fn with_custom_format(self, format: &str) -> DockerVersionCmd<WithCustomFormat> {
        let mut cmd = self.cmd;
        cmd.arg("--format");
        cmd.arg(format);
        DockerVersionCmd {
            cmd,
            _format: PhantomData,
        }
    }
}

/// A trait that represents a format option for the `docker version` command.
pub trait FormatOpt: _priv::Sealed {}

pub struct NoFormatOpt;

impl FormatOpt for NoFormatOpt {}
impl _priv::Sealed for NoFormatOpt {}

pub struct JsonFormatOpt;

impl FormatOpt for JsonFormatOpt {}
impl _priv::Sealed for JsonFormatOpt {}

pub struct WithCustomFormat;

impl FormatOpt for WithCustomFormat {}
impl _priv::Sealed for WithCustomFormat {}

mod _priv {
    #![allow(dead_code)]
    pub trait Sealed {}
}
