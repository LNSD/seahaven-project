/// Represents the content of a Seahaven setup description file.
///
/// This struct follows the compose file format with top-level elements
/// for services, networks, volumes, configs, and secrets. The `services`
/// field is required while other elements are optional.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Content {
    /// Name top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Services top-level element
    pub services: serde_yaml::Mapping,

    /// Init containers top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<serde_yaml::Mapping>,

    /// Networks top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<serde_yaml::Mapping>,

    /// Volumes top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<serde_yaml::Mapping>,

    /// Configs top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<serde_yaml::Mapping>,

    /// Secrets top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_yaml::Mapping>,
}
