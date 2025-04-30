//! Deserializing TOML into a [Manifest] struct.

use super::parts::Manifest;

/// Deserializes a string into a [Manifest].
pub fn from_str(s: &str) -> Result<Manifest, DeserializationError> {
    toml::from_str(s).map_err(DeserializationError)
}

/// The error that occurs when deserializing a string into a [Manifest].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializationError(toml::de::Error);
