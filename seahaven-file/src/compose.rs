//! # Compose file
//!
//! This module provides functionality for working with Compose files.
//! It defines the structure of a Compose file with its top-level elements
//! (services, networks, volumes, configs, and secrets) and provides
//! serialization/deserialization capabilities.
//!
//! The [`ComposeFile`] struct represents a Compose file that follows
//! the Compose specification format, allowing for configuration of multi-container
//! Docker applications.
//!
//! ## References
//!
//! - [Compose Spec](https://github.com/compose-spec/compose-spec/blob/main/spec.md)
//! - [Docker Compose File Reference](https://docs.docker.com/reference/compose-file/)

/// A compose file
///
/// This struct represents a Compose file that follows the Compose specification
/// format, allowing for configuration and operation of multi-container applications.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ComposeFile {
    /// Name top-level element
    ///
    /// Ref: [Name top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#name-top-level-element)
    #[cfg_attr(test, serde(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Services top-level element
    ///
    /// Ref: [Services top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#services-top-level-element)
    pub services: serde_yaml::Mapping,

    /// Networks top-level element
    ///
    /// Ref: [Networks top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#networks-top-level-element)
    #[cfg_attr(test, serde(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<serde_yaml::Mapping>,

    /// Volumes top-level element
    ///
    /// Ref: [Volumes top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#volumes-top-level-element)
    #[cfg_attr(test, serde(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<serde_yaml::Mapping>,

    /// Configs top-level element
    ///
    /// Ref: [Configs top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#configs-top-level-element)
    #[cfg_attr(test, serde(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<serde_yaml::Mapping>,

    /// Secrets top-level element
    ///
    /// Ref: [Secrets top-level element](https://github.com/compose-spec/compose-spec/blob/main/spec.md#secrets-top-level-element)
    #[cfg_attr(test, serde(default))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_yaml::Mapping>,
}

/// Module containing serialization functionality
pub mod ser {
    use super::ComposeFile;

    /// An error that occurs when serializing a compose file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct SerializationError(#[from] pub(crate) serde_yaml::Error);

    /// Converts a [`ComposeFile`] to a string representation.
    ///
    /// Returns a [`SerializationError`] if the serialization fails.
    pub fn to_string(file: &ComposeFile) -> Result<String, SerializationError> {
        serde_yaml::to_string(file).map_err(SerializationError)
    }

    /// Writes a [`ComposeFile`] to the provided writer.
    ///
    /// Returns a [`SerializationError`] if the write operation fails.
    pub fn to_writer<W>(writer: W, file: &ComposeFile) -> Result<(), SerializationError>
    where
        W: std::io::Write,
    {
        serde_yaml::to_writer(writer, file).map_err(SerializationError)
    }
}

/// Module containing deserialization functionality
pub mod de {
    use super::ComposeFile;

    /// An error that occurs when deserializing a compose file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct DeserializationError(#[from] pub(crate) serde_yaml::Error);

    /// Parses a string into a [`ComposeFile`]
    ///
    /// Returns a [`DeserializationError`] if the parsing fails.
    pub fn from_str(s: &str) -> Result<ComposeFile, DeserializationError> {
        serde_yaml::from_str(s).map_err(DeserializationError)
    }

    /// Reads a [`ComposeFile`] from the provided reader
    ///
    /// Returns a [`DeserializationError`] if the reading operation fails.
    pub fn from_reader<R>(reader: R) -> Result<ComposeFile, DeserializationError>
    where
        R: std::io::Read,
    {
        serde_yaml::from_reader(reader).map_err(DeserializationError)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, ErrorKind, Write};

    use testlib_file_testdata::setup_yaml::SINGLE_SERVICE;

    use super::{
        de::{DeserializationError, from_reader, from_str},
        ser::{SerializationError, to_string, to_writer},
    };

    #[test]
    fn serialize_compose_file_to_string() {
        //* Given
        let compose_file =
            serde_yaml::from_str(SINGLE_SERVICE).expect("failed to deserialize compose file");

        //* When
        let serialized = to_string(&compose_file).expect("failed to serialize compose file");

        //* Then
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], SINGLE_SERVICE);
    }

    #[test]
    fn serialize_compose_file_to_writer() {
        //* Given
        let compose_file =
            serde_yaml::from_str(SINGLE_SERVICE).expect("failed to deserialize compose file");

        let mut writer = Vec::new();

        //* When
        to_writer(&mut writer, &compose_file).expect("failed to serialize compose file");

        //* Then
        let serialized =
            String::from_utf8(writer).expect("failed to convert serialized bytes to string");

        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], SINGLE_SERVICE);
    }

    #[test]
    fn serialize_compose_file_to_writer_fails_on_write_error() {
        //* Given
        let compose_file =
            serde_yaml::from_str(SINGLE_SERVICE).expect("failed to deserialize compose file");

        // Create a writer that will fail on write
        struct FailingWriter;
        impl Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    ErrorKind::Other,
                    "simulated write failure",
                ))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let writer = FailingWriter;

        //* When
        let result = to_writer(writer, &compose_file);

        //* Then
        assert!(result.is_err(), "Expected the serialization to fail");
        assert!(
            matches!(result, Err(SerializationError(_))),
            "Expected SerializationError"
        );
    }

    #[test]
    fn deserialize_compose_file_from_str() {
        //* Given
        let compose_yaml = SINGLE_SERVICE;

        //* When
        let compose_file = from_str(compose_yaml).expect("Failed to deserialize compose file");

        //* Then
        assert!(compose_file.name.is_none());
        assert_eq!(compose_file.services.len(), 1);
        assert!(compose_file.networks.is_none());
        assert!(compose_file.volumes.is_none());
        assert!(compose_file.configs.is_none());
        assert!(compose_file.secrets.is_none());
    }

    #[test]
    fn deserialize_compose_file_from_reader() {
        //* Given
        let reader = Cursor::new(SINGLE_SERVICE);

        //* When
        let compose_file = from_reader(reader).expect("Failed to deserialize compose file");

        //* Then
        assert!(compose_file.name.is_none());
        assert_eq!(compose_file.services.len(), 1);
        assert!(compose_file.networks.is_none());
        assert!(compose_file.volumes.is_none());
        assert!(compose_file.configs.is_none());
        assert!(compose_file.secrets.is_none());
    }

    #[test]
    fn deserialize_invalid_compose_file_returns_error() {
        //* Given
        let invalid_yaml = "services: - invalid yaml format";

        //* When
        let result = from_str(invalid_yaml);

        //* Then
        assert!(result.is_err(), "Expected the deserialization to fail");
        assert!(
            matches!(result, Err(DeserializationError(_))),
            "Expected DeserializationError"
        );
    }

    #[test]
    fn deserialize_compose_file_from_reader_fails_on_read_error() {
        //* Given
        // Create a reader that will fail on read
        struct FailingReader;
        impl std::io::Read for FailingReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    ErrorKind::Other,
                    "simulated read failure",
                ))
            }
        }

        let reader = FailingReader;

        //* When
        let result = from_reader(reader);

        //* Then
        assert!(result.is_err(), "Expected the deserialization to fail");
        assert!(
            matches!(result, Err(DeserializationError(_))),
            "Expected DeserializationError"
        );
    }

    #[test]
    fn roundtrip_serialization_deserialization() {
        //* Given
        let original_compose_file =
            from_str(SINGLE_SERVICE).expect("Failed to deserialize compose file");

        //* When
        let serialized =
            to_string(&original_compose_file).expect("Failed to serialize compose file");
        let deserialized_compose_file =
            from_str(&serialized).expect("Failed to deserialize compose file");

        //* Then
        assert_eq!(original_compose_file, deserialized_compose_file);
    }
}
