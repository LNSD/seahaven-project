use std::collections::HashMap;

use super::name::Name;

/// A service in the package
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Service {
    /// The name of the service
    pub name: Name,

    /// The service defaults
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<toml::Value>,

    /// The rest of the service configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub _rest: Option<HashMap<String, toml::Value>>,
}

/// An init container in the package
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InitContainer {
    /// The name of the init container
    pub name: Name,

    /// The init container defaults
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<toml::Value>,

    /// The rest of the init container configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub _rest: Option<HashMap<String, toml::Value>>,
}
