use super::name::Name;

/// A service target in the package
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    /// The name of the target
    pub name: Name,

    /// The service defaults
    #[serde(default, skip_serializing_if = "toml::Table::is_empty")]
    pub defaults: toml::Table,

    /// The rest of the service configuration
    #[serde(default, skip_serializing_if = "toml::Table::is_empty")]
    #[serde(flatten)]
    pub _rest: toml::Table,
}

/// An init container target in the package
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InitContainer {
    /// The name of the init container
    pub name: Name,

    /// The init container defaults
    #[serde(default, skip_serializing_if = "toml::Table::is_empty")]
    pub defaults: toml::Table,

    /// The rest of the init container configuration
    #[serde(default, skip_serializing_if = "toml::Table::is_empty")]
    #[serde(flatten)]
    pub _rest: toml::Table,
}
