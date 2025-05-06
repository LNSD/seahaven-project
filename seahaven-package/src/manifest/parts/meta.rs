use super::name::Name;

/// The package meta information
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PackageMeta {
    /// The name of the package
    ///
    /// This is the name of the package as it will be referenced in the workspace.
    pub name: Name,

    /// The version of the package
    ///
    /// This is an optional field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// The description of the package
    ///
    /// This is an optional field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The readme file for the package
    ///
    /// This is an optional field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
}

impl PackageMeta {
    /// Creates a new package meta information with the given name
    pub fn new(name: Name) -> Self {
        Self {
            name,
            version: None,
            description: None,
            readme: None,
        }
    }
}
