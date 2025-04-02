//! # Seahaven file

use super::{compose_file::ComposeFile, env_file::EnvFile};

/// The setup file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
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

impl File {
    /// Process the file and convert into a [`ComposeFile`] and an [`EnvFile`] pair
    pub fn process(self) -> (ComposeFile, EnvFile) {
        let env_file = EnvFile::new();
        let compose_file = ComposeFile {
            name: self.name,
            services: self.services,
            networks: self.networks,
            volumes: self.volumes,
            configs: self.configs,
            secrets: self.secrets,
        };

        (compose_file, env_file)
    }
}

/// An error that occurs when deserializing a file
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializationError(serde_yaml::Error);

/// An error that occurs when serializing a file
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializationError(serde_yaml::Error);

/// Serialize a file to a string
pub fn serialize_to_string(file: &File) -> Result<String, SerializationError> {
    serde_yaml::to_string(file).map_err(SerializationError)
}

/// Serialize a file to a writer
pub fn serialize_to_writer(
    file: &File,
    writer: impl std::io::Write,
) -> Result<(), SerializationError> {
    serde_yaml::to_writer(writer, file).map_err(SerializationError)
}

/// Deserialize a file from a string
pub fn deserialize_from_str(s: &str) -> Result<File, DeserializationError> {
    serde_yaml::from_str(s).map_err(DeserializationError)
}

/// Deserialize a file from a reader
pub fn deserialize_from_reader(reader: impl std::io::Read) -> Result<File, DeserializationError> {
    serde_yaml::from_reader(reader).map_err(DeserializationError)
}

#[cfg(test)]
mod tests {
    use testlib_file_testdata::setup_yaml::GETTINGSTARTED;

    use super::*;

    #[test]
    fn deserialize_setup_file_from_str() {
        //* Given
        let setup_file = GETTINGSTARTED;

        //* When
        let file = deserialize_from_str(setup_file).expect("Failed to deserialize setup file");

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
        let file = deserialize_from_reader(reader).expect("Failed to deserialize setup file");

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
        let setup_file =
            deserialize_from_str(GETTINGSTARTED).expect("Failed to deserialize setup file");

        //* When
        let serialized = serialize_to_string(&setup_file).expect("Failed to serialize setup file");

        //* Then
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }

    #[test]
    fn serialize_setup_file_to_writer() {
        //* Given
        let setup_file =
            deserialize_from_str(GETTINGSTARTED).expect("Failed to deserialize setup file");

        let mut writer = Vec::new();

        //* When
        serialize_to_writer(&setup_file, &mut writer).expect("Failed to serialize setup file");

        //* Then
        let serialized =
            String::from_utf8(writer).expect("Failed to convert serialized bytes to string");
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }
}
