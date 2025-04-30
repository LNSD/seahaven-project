use serde::de::Error as _;

use super::{
    name::Name,
    services::{InitContainer, Service},
};

/// The manifest for a Seahaven package.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Manifest {
    /// The package meta information
    pub package: PackageMeta,

    /// The service in the package
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,

    /// The init targets in the package
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub init: Vec<InitContainer>,
}

impl<'de> serde::de::Deserialize<'de> for Manifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct ManifestInternal {
            package: PackageMeta,
            #[serde(default)]
            service: Option<Service>,
            #[serde(default)]
            init: Vec<InitContainer>,
        }

        let manifest = ManifestInternal::deserialize(deserializer)?;

        // Validate the manifest
        // Check that there are either a service or init containers defined
        if manifest.service.is_none() && manifest.init.is_empty() {
            return Err(D::Error::custom("no service or init containers defined"));
        }

        Ok(Manifest {
            package: manifest.package,
            service: manifest.service,
            init: manifest.init,
        })
    }
}

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
