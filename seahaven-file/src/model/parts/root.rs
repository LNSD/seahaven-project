//! Represents the content of a Seahaven setup description file.
//!
//! This struct follows the compose file format with top-level elements
//! for services, networks, volumes, configs, and secrets. The `services`
//! field is required while other elements are optional.

use indexmap::IndexMap as Map;

use super::name::Name;

/// Represents the content of a setup file after parsing, and before validation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeserializedRoot {
    /// Name top-level element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,

    /// Services top-level element
    pub services: Map<Name, serde_yaml::Value>,

    /// Init containers top-level element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<Map<Name, serde_yaml::Value>>,

    /// Rest of the file content
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub _rest: Map<String, serde_yaml::Value>,
}

/// Represents the content of a setup file after validation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidatedRoot {
    /// Name top-level element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,

    /// Services top-level element
    pub services: Map<Name, serde_yaml::Value>,

    /// Init containers top-level element
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<Map<Name, serde_yaml::Value>>,

    /// Rest of the file content
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub _rest: Map<String, serde_yaml::Value>,
}
