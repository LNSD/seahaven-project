/// An env file
#[derive(Debug, serde::Serialize)]
pub struct EnvFile(serde_envfile::Value);

impl EnvFile {
    /// Create a new empty env file
    pub fn new() -> Self {
        Self(serde_envfile::Value::new())
    }
}

impl Default for EnvFile {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for EnvFile {
    type Target = serde_envfile::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EnvFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> FromIterator<(K, V)> for EnvFile
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut env_file = EnvFile::new();
        env_file.extend(
            iter.into_iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string())),
        );
        env_file
    }
}

/// An error that occurs when serializing an env file
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializationError(serde_envfile::Error);

pub fn serialize_to_string(file: &EnvFile) -> Result<String, SerializationError> {
    serde_envfile::to_string(file).map_err(SerializationError)
}

pub fn serialize_to_writer(
    _file: &EnvFile,
    _writer: impl std::io::Write,
) -> Result<(), SerializationError> {
    unimplemented!("not supported by serde_envfile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_env_file_to_string() {
        //* Given
        let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

        //* When
        let serialized = serialize_to_string(&env_file).expect("failed to serialize env file");

        //* Then
        // Internally, serde_envfile uses a `HashMap` to store the key-value pairs,
        // so the order of the key-value pairs is not guaranteed
        assert!(serialized.contains(r#"KEY1="VALUE1""#));
        assert!(serialized.contains(r#"KEY2="VALUE2""#));
    }

    #[test]
    #[should_panic = "not supported by serde_envfile"]
    fn serialize_env_file_to_writer() {
        //* Given
        let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

        //* When
        serialize_to_writer(&env_file, &mut Vec::new()).expect("failed to serialize env file");

        //* Then
        // This should never be reached
    }
}
