//! Environment file handling
//!
//! This module provides functionality for working with environment files,
//! including parsing, creating, and manipulating environment variables
//! stored in an envfile format.

/// Represents an environment file containing key-value pairs
///
/// This struct provides a convenient way to work with environment variables
/// stored in a file format. It wraps a `serde_envfile::Value` and provides
/// methods to create, manipulate, and access environment variables.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EnvFile(serde_envfile::Value);

impl EnvFile {
    /// Creates a new empty environment file.
    ///
    /// Returns an `EnvFile` instance with no key-value pairs.
    pub fn new() -> Self {
        Self(serde_envfile::Value::new())
    }

    /// Returns an iterator over references to key-value pairs in this environment file.
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    /// Returns an iterator over mutable references to key-value pairs in this environment file.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        self.into_iter()
    }

    /// Inserts a key-value pair into the environment file.
    ///
    /// If the key already exists, the value is replaced and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::env::EnvFile;
    ///
    /// let mut env_file = EnvFile::new();
    /// assert_eq!(env_file.insert("KEY1", "VALUE1"), None);
    /// assert_eq!(
    ///     env_file.insert("KEY1", "NEW_VALUE"),
    ///     Some("VALUE1".to_string())
    /// );
    /// ```
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.0.insert(key.into(), value.into())
    }

    /// Removes a key-value pair from the environment file.
    ///
    /// Returns the value if the key was present, or `None` if it was not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::env::EnvFile;
    ///
    /// let mut env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// assert_eq!(env_file.remove("KEY1"), Some("VALUE1".to_string()));
    /// assert_eq!(env_file.remove("KEY3"), None);
    /// ```
    pub fn remove<K>(&mut self, key: K) -> Option<String>
    where
        K: Into<String>,
    {
        self.0.remove(key.into())
    }

    /// Removes a key-value pair from the environment file and returns it as a tuple.
    ///
    /// Returns the key-value pair if the key was present, or `None` if it was not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::env::EnvFile;
    ///
    /// let mut env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// assert_eq!(
    ///     env_file.remove_entry("KEY1"),
    ///     Some(("KEY1".to_string(), "VALUE1".to_string()))
    /// );
    /// assert_eq!(env_file.remove_entry("KEY3"), None);
    /// ```
    pub fn remove_entry<K>(&mut self, key: K) -> Option<(String, String)>
    where
        K: Into<String>,
    {
        let key = key.into();
        self.0.remove(&key).map(|value| (key, value))
    }
}

impl Default for EnvFile {
    /// Creates a default `EnvFile` instance.
    ///
    /// This is equivalent to calling `EnvFile::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for EnvFile {
    type Target = serde_envfile::Value;

    /// Provides immutable access to the underlying map
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EnvFile {
    /// Provides mutable access to the underlying map
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> FromIterator<(K, V)> for EnvFile
where
    K: Into<String>,
    V: Into<String>,
{
    /// Creates an `EnvFile` from an iterator of key-value pairs.
    ///
    /// This allows constructing an environment file from any collection of string-like key-value pairs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::env::EnvFile;
    ///
    /// let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// # assert_eq!(env_file.get("KEY1").unwrap(), "VALUE1");
    /// # assert_eq!(env_file.get("KEY2").unwrap(), "VALUE2");
    /// ```
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}

impl<K, V> Extend<(K, V)> for EnvFile
where
    K: Into<String>,
    V: Into<String>,
{
    /// Extends an `EnvFile` with key-value pairs from an iterator.
    ///
    /// This allows adding multiple key-value pairs to an existing environment file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::env::EnvFile;
    ///
    /// let mut env_file = EnvFile::new();
    /// env_file.extend([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// # assert_eq!(env_file.get("KEY1").unwrap(), "VALUE1");
    /// # assert_eq!(env_file.get("KEY2").unwrap(), "VALUE2");
    /// ```
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

/// Enables iteration over references to key-value pairs in an `EnvFile`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::env::EnvFile;
///
/// let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in &env_file {
///     println!("{} = {}", key, value);
/// }
/// ```
impl<'a> IntoIterator for &'a EnvFile {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

/// Enables iteration over mutable references to key-value pairs in an `EnvFile`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::env::EnvFile;
///
/// let mut env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in &mut env_file {
///     *value = format!("{}_UPDATED", value);
/// }
/// ```
impl<'a> IntoIterator for &'a mut EnvFile {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.0.iter_mut())
    }
}

/// Enables iteration over owned key-value pairs from an `EnvFile`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::env::EnvFile;
///
/// let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in env_file {
///     println!("{} = {}", key, value);
/// }
/// ```
impl IntoIterator for EnvFile {
    type Item = (String, String);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// Iterator over references to key-value pairs in an [`EnvFile`]
#[derive(Debug)]
pub struct Iter<'a>(serde_envfile::value::Iter<'a>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Iterator over mutable references to key-value pairs in an [`EnvFile`]
#[derive(Debug)]
pub struct IterMut<'a>(serde_envfile::value::IterMut<'a>);

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a String, &'a mut String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Iterator over owned key-value pairs from an [`EnvFile`]
#[derive(Debug)]
pub struct IntoIter(serde_envfile::value::IntoIter);

impl Iterator for IntoIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Module containing serialization functionality
pub mod ser {
    use super::EnvFile;

    /// An error that occurs when serializing an env file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct SerializationError(#[from] serde_envfile::Error);

    /// Converts an [`EnvFile`] to a string representation.
    ///
    /// Returns a [`SerializationError`] if the serialization fails.
    pub fn to_string(file: &EnvFile) -> Result<String, SerializationError> {
        serde_envfile::to_string(file).map_err(SerializationError)
    }

    /// Writes an [`EnvFile`] to the provided writer.
    ///
    /// Returns a [`SerializationError`] if the write operation fails.
    pub fn to_writer<W>(writer: W, file: &EnvFile) -> Result<(), SerializationError>
    where
        W: std::io::Write,
    {
        serde_envfile::to_writer(writer, file).map_err(SerializationError)
    }
}

/// Module containing deserialization functionality
pub mod de {
    use super::EnvFile;

    /// An error that occurs when deserializing an env file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct DeserializationError(#[from] serde_envfile::Error);

    /// Parses a string into an [`EnvFile`]
    ///
    /// Returns a [`DeserializationError`] if the parsing fails.
    pub fn from_string(string: &str) -> Result<EnvFile, DeserializationError> {
        serde_envfile::from_str(string).map_err(DeserializationError)
    }

    /// Reads an [`EnvFile`] from the provided reader
    ///
    /// Returns a [`DeserializationError`] if the reading operation fails.
    pub fn from_reader<R>(reader: R) -> Result<EnvFile, DeserializationError>
    where
        R: std::io::Read,
    {
        serde_envfile::from_reader(reader).map_err(DeserializationError)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{
        EnvFile,
        de::{from_reader, from_string},
        ser::{to_string, to_writer},
    };

    #[test]
    fn serialize_env_file_to_string() {
        //* Given
        let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

        //* When
        let serialized = to_string(&env_file).expect("failed to serialize env file");

        //* Then
        let expected = "KEY1=\"VALUE1\"\nKEY2=\"VALUE2\"";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_env_file_to_writer() {
        //* Given
        let env_file = EnvFile::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

        let mut buffer = Vec::new();

        //* When
        to_writer(&mut buffer, &env_file).expect("Failed to serialize env file");

        //* Then
        let expected_string = "KEY1=\"VALUE1\"\nKEY2=\"VALUE2\"";
        let buffer_string = String::from_utf8(buffer).expect("Invalid UTF-8 buffer");
        assert_eq!(buffer_string, expected_string);
    }

    #[test]
    fn deserialize_env_file_from_string() {
        //* Given
        let env_string = "KEY1=\"VALUE1\"\nKEY2=\"VALUE2\"";

        //* When
        let env_file = from_string(env_string).expect("Failed to deserialize env file");

        //* Then
        // There is a bug in serde_envfile that causes the keys to be converted to lowercase
        let expected_env_file = EnvFile::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);
        assert_eq!(env_file, expected_env_file);
    }

    #[test]
    fn deserialize_env_file_from_reader() {
        //* Given
        let env_string = "KEY1=\"VALUE1\"\nKEY2=\"VALUE2\"";
        let reader = Cursor::new(env_string);

        //* When
        let env_file = from_reader(reader).expect("Failed to deserialize env file");

        //* Then
        // There is a bug in serde_envfile that causes the keys to be converted to lowercase
        let expected_env_file = EnvFile::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);
        assert_eq!(env_file, expected_env_file);
    }

    #[test]
    fn deserialize_invalid_env_file_returns_error() {
        //* Given
        let invalid_env_string = "KEY1=VALUE1\nINVALID_LINE\nKEY2=VALUE2";

        //* When
        let result = from_string(invalid_env_string);

        //* Then
        assert!(result.is_err(), "Expected the deserialization to fail");
    }

    #[test]
    fn roundtrip_serialization_deserialization() {
        //* Given
        // There is a bug in serde_envfile that causes the keys to be converted to lowercase
        let original_env_file = EnvFile::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);

        //* When
        let serialized = to_string(&original_env_file).expect("Failed to serialize env file");
        let deserialized_env_file =
            from_string(&serialized).expect("Failed to deserialize env file");

        //* Then
        assert_eq!(original_env_file, deserialized_env_file);
    }
}
