use super::name::Name;

/// The package meta information
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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
