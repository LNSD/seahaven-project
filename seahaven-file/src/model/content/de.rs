use super::Content;

/// An error that happened when deserializing the setup description file content
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializationError(#[from] serde_yaml::Error);

/// Deserialize a setup description file content from a string
pub fn from_str(s: &str) -> Result<Content, DeserializationError> {
    serde_yaml::from_str(s).map_err(DeserializationError)
}

/// Deserialize a setup description file content from a reader
pub fn from_reader<R>(reader: R) -> Result<Content, DeserializationError>
where
    R: std::io::Read,
{
    serde_yaml::from_reader(reader).map_err(DeserializationError)
}
