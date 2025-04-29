use indexmap::IndexMap as Map;

/// Represents a service in the setup file
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Service {
    /// The rest of the service content
    #[serde(flatten)]
    pub _rest: Map<String, serde_yaml::Value>,
}

/// Represents a init-container service in the setup file
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InitContainer {
    /// The rest of the init-container content
    #[serde(flatten)]
    pub _rest: Map<String, serde_yaml::Value>,
}
