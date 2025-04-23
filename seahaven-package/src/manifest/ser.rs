//! Serializing a [Manifest] into TOML.

use super::model::Manifest;

/// Serializes a [Manifest] as a String of TOML.
pub fn to_string(manifest: &Manifest) -> Result<String, SerializationError> {
    toml::to_string(manifest).map_err(SerializationError)
}

/// Serializes a [Manifest] as a "pretty" String of TOML.
pub fn to_pretty_string(manifest: &Manifest) -> Result<String, SerializationError> {
    toml::to_string_pretty(manifest).map_err(SerializationError)
}

/// The error that occurs when serializing a [Manifest] as a String of TOML.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializationError(toml::ser::Error);
