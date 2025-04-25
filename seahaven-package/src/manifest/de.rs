//! Deserializing TOML into a [Manifest] struct.

use serde::de::Error;

use super::model::{InitContainer, Manifest, Name, PackageMeta, Service};

/// Deserializes a string into a [Manifest].
pub fn from_str(s: &str) -> Result<Manifest, DeserializationError> {
    toml::from_str(s).map_err(DeserializationError)
}

/// The error that occurs when deserializing a string into a [Manifest].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializationError(toml::de::Error);

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

impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        if !validate::is_valid_name(&name) {
            return Err(D::Error::custom(format!("invalid name: '{}'", name)));
        }
        Ok(Name::new_unchecked(name))
    }
}

mod validate {
    use once_cell::sync::Lazy;

    /// Validates that a name is valid.
    ///
    /// A name is valid if it matches the `^[a-zA-Z0-9._-]+$` regex.
    pub fn is_valid_name(name: &str) -> bool {
        static NAME_REGEX: Lazy<regress::Regex> =
            Lazy::new(|| regress::Regex::new(r#"^[a-zA-Z0-9._-]+$"#).expect("Invalid regex"));

        NAME_REGEX.find(name).is_some()
    }
}
