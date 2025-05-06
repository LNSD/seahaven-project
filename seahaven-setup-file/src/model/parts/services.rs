use super::{common::FromStructOrString, package_use::PackageUse};

/// Represents a service in the setup file
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Service {
    /// A package `use` binding
    #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<FromStructOrString>")]
    pub package_use: Option<PackageUse>,

    /// The rest of the service content
    #[serde(flatten)]
    pub _rest: serde_yaml::Mapping,
}

/// Represents a init-container service in the setup file
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InitContainer {
    /// A package `use` binding
    #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<FromStructOrString>")]
    pub package_use: Option<PackageUse>,

    /// The rest of the init-container content
    #[serde(flatten)]
    pub _rest: serde_yaml::Mapping,
}
