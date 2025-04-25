/// The manifest for a Seahaven package.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct PackageMeta {
    /// The name of the package
    ///
    /// This is the name of the package as it will be referenced in the workspace.
    pub name: Name,

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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct Service {
    /// The name of the service
    pub name: Name,

    /// The service defaults
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub defaults: Option<toml::Value>,
}

/// An init container in the package
#[derive(Debug, Clone)]
#[cfg_attr(feature = "parse", derive(serde::Deserialize))]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
pub struct InitContainer {
    /// The name of the init container
    pub name: Name,

    /// The init container defaults
    #[cfg_attr(feature = "parse", serde(default))]
    #[cfg_attr(feature = "display", serde(skip_serializing_if = "Option::is_none"))]
    pub defaults: Option<toml::Value>,
}

/// A name that is validated against the `^[a-zA-Z0-9._-]+$` regex.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "display", derive(serde::Serialize))]
#[cfg_attr(any(feature = "parse", feature = "display"), serde(transparent))]
pub struct Name(String);

impl Name {
    /// Creates a new name from a [`String`].
    ///
    /// This is an internal function that does not validate the name.
    #[cfg(feature = "parse")]
    pub(super) fn new_unchecked(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Consumes the [`Name`] and returns the inner [`String`].
    ///
    /// The returned [`String`] is guaranteed to be valid.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> std::cmp::PartialEq<T> for Name
where
    T: ?Sized + std::convert::AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.as_str() == other.as_ref()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
