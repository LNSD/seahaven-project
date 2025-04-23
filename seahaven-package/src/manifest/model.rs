/// The manifest for a Seahaven package.
#[derive(Debug)]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct Manifest {
    /// The package meta information
    pub package: PackageMeta,

    /// The service in the package
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub service: Option<Service>,

    /// The init targets in the package
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Vec::is_empty"))]
    pub init: Vec<InitContainer>,
}

/// The package meta information
#[derive(Debug)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct PackageMeta {
    /// The name of the package
    ///
    /// This is the name of the package as it will be referenced in the workspace.
    pub name: String,

    /// The version of the package
    ///
    /// This is an optional field.
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    /// The description of the package
    ///
    /// This is an optional field.
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// The readme file for the package
    ///
    /// This is an optional field.
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub readme: Option<String>,
}

/// A service in the package
#[derive(Debug)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct Service {
    /// The name of the service
    pub name: String,

    /// The service defaults
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub defaults: Option<toml::Value>,
}

/// An init container in the package
#[derive(Debug)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct InitContainer {
    /// The name of the init container
    pub name: String,

    /// The init container defaults
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub defaults: Option<toml::Value>,
}
