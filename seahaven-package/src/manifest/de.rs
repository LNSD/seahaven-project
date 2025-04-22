//! Deserializing TOML into a [Manifest] struct.

use serde::de::Error;

use super::{InitContainer, Manifest, PackageMeta, Service};

/// Deserializes a string into a [Manifest].
pub fn from_str(s: &str) -> Result<Manifest, DeserializationError> {
    toml::from_str(s).map_err(DeserializationError)
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
            services: Vec<Service>,
            #[serde(default)]
            init: Vec<InitContainer>,
        }

        let manifest = ManifestInternal::deserialize(deserializer)?;

        // Validate the manifest
        let name_regex = regress::Regex::new(r#"^[a-zA-Z0-9._-]+$"#).expect("Invalid name regex");

        // Check that the package name is valid
        if name_regex.find(&manifest.package.name).is_none() {
            return Err(D::Error::custom(format!(
                "invalid package name: '{}'",
                manifest.package.name
            )));
        }

        // Check that there are either services or init containers defined
        if manifest.services.is_empty() && manifest.init.is_empty() {
            return Err(D::Error::custom("no services or init containers defined"));
        }

        // Check that all service names are valid
        for service in &manifest.services {
            if name_regex.find(&service.name).is_none() {
                return Err(D::Error::custom(format!(
                    "invalid service name: '{}'",
                    service.name
                )));
            }
        }

        // Check that all init container names are valid
        for init in &manifest.init {
            if name_regex.find(&init.name).is_none() {
                return Err(D::Error::custom(format!(
                    "invalid init container name: '{}'",
                    init.name
                )));
            }
        }

        Ok(Manifest {
            package: manifest.package,
            services: manifest.services,
            init: manifest.init,
        })
    }
}

/// The error that occurs when deserializing a string into a [Manifest].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializationError(toml::de::Error);
