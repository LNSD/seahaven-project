use super::Content;

/// An error that happened when serializing the setup description file content
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializationError(#[from] serde_yaml::Error);

/// Serialize a setup description file content to a string
pub fn to_string(file: &Content) -> Result<String, SerializationError> {
    serde_yaml::to_string(file).map_err(SerializationError)
}

/// Serialize a setup description file content to a writer
pub fn to_writer<W>(writer: W, file: &Content) -> Result<(), SerializationError>
where
    W: std::io::Write,
{
    serde_yaml::to_writer(writer, file).map_err(SerializationError)
}
