use super::{IntoCmdOptValue, IntoCommand};

pub struct DockerComposeBuildCmd<D = DryRunNotSet, B = BuildArgsNotSet, S = ServicesNotSet> {
    cmd: tokio::process::Command,
    dry_run_opt: D,
    build_args_opt: B,
    services_opt: S,
}

impl DockerComposeBuildCmd {
    pub(crate) fn new(cmd: impl IntoCommand) -> Self {
        Self {
            cmd: cmd.into_command(),
            dry_run_opt: DryRunNotSet,
            build_args_opt: BuildArgsNotSet,
            services_opt: ServicesNotSet,
        }
    }
}

impl<D, B, S> IntoCommand for DockerComposeBuildCmd<D, B, S>
where
    D: DryRunOpt,
    B: BuildArgsOpt,
    S: ServicesOpt,
{
    fn into_command(self) -> tokio::process::Command {
        let mut cmd = self.cmd;

        // Add the `build` subcommand
        cmd.arg("build");

        // --dry-run
        if matches!(self.dry_run_opt.into_value(), Some(true)) {
            cmd.arg("--dry-run");
        }

        // --build-arg <key>=<value>
        if let Some(build_args) = self.build_args_opt.into_value() {
            for (key, value) in build_args {
                cmd.arg("--build-arg").arg(format!("{}={}", key, value));
            }
        }

        // [SERVICES...]
        if let Some(services) = self.services_opt.into_value() {
            cmd.args(services);
        }
        cmd
    }
}

impl<D, S> DockerComposeBuildCmd<D, BuildArgsNotSet, S>
where
    D: DryRunOpt,
    S: ServicesOpt,
{
    /// Add multiple build arguments.
    ///
    /// If the build argument key is empty, the key-value pair will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn with_build_args<I, K, V>(self, args: I) -> DockerComposeBuildCmd<D, BuildArgsSet, S>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let build_args = args
            .into_iter()
            .filter_map(|(key, value)| {
                // Filter out empty key build arguments
                let key = key.as_ref();
                let value = value.as_ref();
                if key.is_empty() {
                    None
                } else {
                    Some((key.to_string(), value.to_string()))
                }
            })
            .collect();

        DockerComposeBuildCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            build_args_opt: BuildArgsSet(build_args),
            services_opt: self.services_opt,
        }
    }

    /// Add a build argument in the form of `<key>=<value>`.
    ///
    /// If the build argument key is empty, the key-value pair will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn with_build_arg<K, V>(self, key: K, value: V) -> DockerComposeBuildCmd<D, BuildArgsSet, S>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.with_build_args([(key, value)])
    }
}

impl<D, S> DockerComposeBuildCmd<D, BuildArgsSet, S>
where
    D: DryRunOpt,
    S: ServicesOpt,
{
    pub fn with_build_arg<K, V>(self, key: K, value: V) -> DockerComposeBuildCmd<D, BuildArgsSet, S>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let BuildArgsSet(mut args) = self.build_args_opt;

        // Filter out empty key build arguments
        if !key.as_ref().is_empty() {
            args.push((key.as_ref().to_string(), value.as_ref().to_string()));
        }

        DockerComposeBuildCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            build_args_opt: BuildArgsSet(args),
            services_opt: self.services_opt,
        }
    }
}

impl<D, B> DockerComposeBuildCmd<D, B, ServicesNotSet>
where
    D: DryRunOpt,
    B: BuildArgsOpt,
{
    /// Specify which services to build.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn with_services<I, T>(self, services: I) -> DockerComposeBuildCmd<D, B, ServicesSet>
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

        DockerComposeBuildCmd {
            cmd: self.cmd,
            dry_run_opt: self.dry_run_opt,
            build_args_opt: self.build_args_opt,
            services_opt: ServicesSet(services),
        }
    }

    /// Specify a single service to build.
    ///
    /// If the service name is empty, it will be ignored.
    ///
    /// See the [Docker Compose documentation](https://docs.docker.com/compose/reference/build/)
    /// for more information.
    pub fn with_service<T>(self, service: T) -> DockerComposeBuildCmd<D, B, ServicesSet>
    where
        T: AsRef<str>,
    {
        self.with_services([service])
    }
}

/// Marker trait for dry-run option
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

/// Marker trait for build arguments options
#[allow(private_bounds)]
pub trait BuildArgsOpt: IntoCmdOptValue<Vec<(String, String)>> + _priv::Sealed {}

pub struct BuildArgsNotSet;
impl BuildArgsOpt for BuildArgsNotSet {}
impl _priv::Sealed for BuildArgsNotSet {}

impl IntoCmdOptValue<Vec<(String, String)>> for BuildArgsNotSet {
    fn into_value(self) -> Option<Vec<(String, String)>> {
        None
    }
}

pub struct BuildArgsSet(pub(super) Vec<(String, String)>);

impl BuildArgsOpt for BuildArgsSet {}
impl _priv::Sealed for BuildArgsSet {}

impl IntoCmdOptValue<Vec<(String, String)>> for BuildArgsSet {
    fn into_value(self) -> Option<Vec<(String, String)>> {
        Some(self.0)
    }
}

/// Marker trait for services options
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
