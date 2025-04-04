//! # Seahaven setup description file contenta
//!
//! This module provides types and functions for working with Seahaven setup description files.
//! It includes the `FileContent` struct that represents the content of a setup file,
//! along with serialization and deserialization utilities.

/// Represents the content of a Seahaven setup description file.
///
/// This struct follows the compose file format with top-level elements
/// for services, networks, volumes, configs, and secrets. The `services`
/// field is required while other elements are optional.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    /// Name top-level element
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Services top-level element
    pub services: serde_yaml::Mapping,

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

/// Module containing serialization functionality
pub mod ser {
    use super::FileContent;

    /// An error that happened when serializing the setup description file content
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct SerializationError(#[from] serde_yaml::Error);

    /// Serialize a setup description file content to a string
    pub fn to_string(file: &FileContent) -> Result<String, SerializationError> {
        serde_yaml::to_string(file).map_err(SerializationError)
    }

    /// Serialize a setup description file content to a writer
    pub fn to_writer<W>(writer: W, file: &FileContent) -> Result<(), SerializationError>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, file).map_err(SerializationError)
    }
}

/// Module containing deserialization functionality
pub mod de {
    use super::FileContent;

    /// An error that happened when deserializing the setup description file content
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct DeserializationError(#[from] serde_yaml::Error);

    /// Deserialize a setup description file content from a string
    pub fn from_str(s: &str) -> Result<FileContent, DeserializationError> {
        serde_yaml::from_str(s).map_err(DeserializationError)
    }

    /// Deserialize a setup description file content from a reader
    pub fn from_reader<R>(reader: R) -> Result<FileContent, DeserializationError>
    where
        R: std::io::Read,
    {
        serde_yaml::from_reader(reader).map_err(DeserializationError)
    }
}

#[cfg(test)]
mod tests {
    use testlib_file_testdata::setup_yaml::GETTINGSTARTED;

    use super::{
        de::{from_reader, from_str},
        ser::{to_string, to_writer},
    };

    #[test]
    fn deserialize_setup_file_from_str() {
        //* Given
        let setup_file = GETTINGSTARTED;

        //* When
        let file = from_str(setup_file).expect("Failed to deserialize setup file");

        //* Then
        assert!(file.name.is_none());
        assert_eq!(file.services.len(), 2);
        assert!(file.networks.is_none());
        assert!(file.volumes.is_none());
        assert!(file.configs.is_none());
        assert!(file.secrets.is_none());
    }

    #[test]
    fn deserialize_setup_file_from_reader() {
        //* Given
        let reader = GETTINGSTARTED.as_bytes();

        //* When
        let file = from_reader(reader).expect("Failed to deserialize setup file");

        //* Then
        assert!(file.name.is_none());
        assert_eq!(file.services.len(), 2);
        assert!(file.networks.is_none());
        assert!(file.volumes.is_none());
        assert!(file.configs.is_none());
        assert!(file.secrets.is_none());
    }

    #[test]
    fn serialize_setup_file_to_string() {
        //* Given
        let setup_file = from_str(GETTINGSTARTED).expect("Failed to deserialize setup file");

        //* When
        let serialized = to_string(&setup_file).expect("Failed to serialize setup file");

        //* Then
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }

    #[test]
    fn serialize_setup_file_to_writer() {
        //* Given
        let setup_file = from_str(GETTINGSTARTED).expect("Failed to deserialize setup file");

        let mut writer = Vec::new();

        //* When
        to_writer(&mut writer, &setup_file).expect("Failed to serialize setup file");

        //* Then
        let serialized =
            String::from_utf8(writer).expect("Failed to convert serialized bytes to string");
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }
}
