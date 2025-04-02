//! # Compose file
//!
//! ## References
//!
//! - [Compose Spec](https://github.com/compose-spec/compose-spec/blob/main/spec.md)
//! - [Docker Compose File Reference](https://docs.docker.com/reference/compose-file/)

/// A compose file
#[derive(Debug, serde::Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
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

/// An error that occurs when serializing a compose file
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializationError(serde_yaml::Error);

/// Serialize a compose file to a string
pub fn serialize_to_string(file: &ComposeFile) -> Result<String, SerializationError> {
    serde_yaml::to_string(file).map_err(SerializationError)
}

/// Serialize a compose file to a writer
pub fn serialize_to_writer(
    file: &ComposeFile,
    writer: &mut impl std::io::Write,
) -> Result<(), SerializationError> {
    serde_yaml::to_writer(writer, file).map_err(SerializationError)
}

#[cfg(test)]
mod tests {
    use testlib_file_testdata::setup_yaml::GETTINGSTARTED;

    use super::*;

    #[test]
    fn serialize_compose_file_to_string() {
        //* Given
        let compose_file =
            serde_yaml::from_str(GETTINGSTARTED).expect("failed to deserialize compose file");

        //* When
        let serialized =
            serialize_to_string(&compose_file).expect("failed to serialize compose file");

        //* Then
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }

    #[test]
    fn serialize_compose_file_to_writer() {
        //* Given
        let compose_file =
            serde_yaml::from_str(GETTINGSTARTED).expect("failed to deserialize compose file");

        let mut writer = Vec::new();

        //* When
        serialize_to_writer(&compose_file, &mut writer).expect("failed to serialize compose file");

        //* Then
        let serialized =
            String::from_utf8(writer).expect("failed to convert serialized bytes to string");

        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], GETTINGSTARTED);
    }
}
