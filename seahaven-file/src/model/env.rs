//! Environment file handling
//!
//! This module provides a robust interface for working with environment files in the
//! standard `.env` format. It offers functionality for:
//!
//! - Parsing environment files from strings or readers
//! - Creating and manipulating environment variables
//! - Serializing environment variables to strings or writers
//! - Iterating over environment variables
//!
//! The module is built on top of the `serde_envfile` crate and provides a more
//! ergonomic API for working with environment files in Rust applications.

/// Represents environment variables
///
/// This struct provides a convenient way to work with environment variables
/// stored in a file format. It wraps a `serde_envfile::Value` and provides
/// methods to create, manipulate, and access environment variables.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Env(serde_envfile::Value);

impl Env {
    /// Creates a new empty environment file.
    ///
    /// Returns an `Env` instance with no key-value pairs.
    pub fn new() -> Self {
        Self(serde_envfile::Value::new())
    }

    /// Returns the number of key-value pairs in the environment file.
    ///
    /// # Examples
    ///
    /// ```
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env = Env::new();
    /// assert_eq!(env.len(), 0);
    ///
    /// env.insert("KEY1", "VALUE1");
    /// assert_eq!(env.len(), 1);
    ///
    /// env.insert("KEY2", "VALUE2");
    /// assert_eq!(env.len(), 2);
    ///
    /// env.remove("KEY1");
    /// assert_eq!(env.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the environment file contains no key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env = Env::new();
    /// assert!(env.is_empty());
    ///
    /// env.insert("KEY", "VALUE");
    /// assert!(!env.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the value associated with the key.
    ///
    /// Returns `None` if the key is not present in the environment file.
    ///
    /// # Examples
    ///
    /// ```
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env = Env::new();
    /// env.insert("DATABASE_URL", "postgres://localhost:5432/mydb");
    ///
    /// let value = env.get("DATABASE_URL");
    /// assert!(value.is_some());
    /// assert_eq!(value.unwrap(), "postgres://localhost:5432/mydb");
    /// assert!(env.get("NONEXISTENT_KEY").is_none());
    /// ```
    pub fn get<K>(&self, key: K) -> Option<&String>
    where
        K: Into<String>,
    {
        self.0.get(&key.into())
    }

    /// Returns a mutable reference to the value associated with the key.
    ///
    /// Returns `None` if the key is not present in the environment file.
    ///
    /// # Examples
    ///
    /// ```
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env = Env::new();
    /// env.insert("DATABASE_URL", "postgres://localhost:5432/mydb");
    ///
    /// if let Some(value) = env.get_mut("DATABASE_URL") {
    ///     *value = "postgres://localhost:5432/newdb".to_string();
    /// }
    ///
    /// let value = env.get("DATABASE_URL");
    /// assert!(value.is_some());
    /// assert_eq!(value.unwrap(), "postgres://localhost:5432/newdb");
    /// ```
    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut String>
    where
        K: Into<String>,
    {
        self.0.get_mut(&key.into())
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
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env_file = Env::new();
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
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
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
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
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

impl Default for Env {
    /// Creates a default `Env` instance.
    ///
    /// This is equivalent to calling `Env::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for Env
where
    K: Into<String>,
    V: Into<String>,
{
    /// Creates an `Env` from an iterator of key-value pairs.
    ///
    /// This allows constructing an environment file from any collection of string-like key-value pairs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::model::env::Env;
    ///
    /// let env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// # assert_eq!(env_file.get("KEY1").unwrap(), "VALUE1");
    /// # assert_eq!(env_file.get("KEY2").unwrap(), "VALUE2");
    /// ```
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}

impl<K, V> Extend<(K, V)> for Env
where
    K: Into<String>,
    V: Into<String>,
{
    /// Extends an `Env` with key-value pairs from an iterator.
    ///
    /// This allows adding multiple key-value pairs to an existing environment file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use seahaven_file::model::env::Env;
    ///
    /// let mut env_file = Env::new();
    /// env_file.extend([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
    /// # assert_eq!(env_file.get("KEY1").unwrap(), "VALUE1");
    /// # assert_eq!(env_file.get("KEY2").unwrap(), "VALUE2");
    /// ```
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

/// Enables iteration over references to key-value pairs in an `Env`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::model::env::Env;
///
/// let env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in &env_file {
///     println!("{} = {}", key, value);
/// }
/// ```
impl<'a> IntoIterator for &'a Env {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

/// Enables iteration over mutable references to key-value pairs in an `Env`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::model::env::Env;
///
/// let mut env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in &mut env_file {
///     *value = format!("{}_UPDATED", value);
/// }
/// ```
impl<'a> IntoIterator for &'a mut Env {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.0.iter_mut())
    }
}

/// Enables iteration over owned key-value pairs from an `Env`.
///
/// # Examples
///
/// ```rust
/// use seahaven_file::model::env::Env;
///
/// let env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);
/// for (key, value) in env_file {
///     println!("{} = {}", key, value);
/// }
/// ```
impl IntoIterator for Env {
    type Item = (String, String);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// Iterator over references to key-value pairs in an [`Env`]
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

/// Iterator over mutable references to key-value pairs in an [`Env`]
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

/// Iterator over owned key-value pairs from an [`Env`]
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
    use super::Env;

    /// An error that occurs when serializing an env file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct SerializationError(#[from] serde_envfile::Error);

    /// Converts an [`Env`] to a string representation.
    ///
    /// Returns a [`SerializationError`] if the serialization fails.
    pub fn to_string(file: &Env) -> Result<String, SerializationError> {
        serde_envfile::to_string(file).map_err(SerializationError)
    }

    /// Writes an [`Env`] to the provided writer.
    ///
    /// Returns a [`SerializationError`] if the write operation fails.
    pub fn to_writer<W>(writer: W, file: &Env) -> Result<(), SerializationError>
    where
        W: std::io::Write,
    {
        serde_envfile::to_writer(writer, file).map_err(SerializationError)
    }
}

/// Module containing deserialization functionality
pub mod de {
    use super::Env;

    /// An error that occurs when deserializing an env file
    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct DeserializationError(#[from] serde_envfile::Error);

    /// Parses a string into an [`Env`]
    ///
    /// Returns a [`DeserializationError`] if the parsing fails.
    pub fn from_string(string: &str) -> Result<Env, DeserializationError> {
        serde_envfile::from_str(string).map_err(DeserializationError)
    }

    /// Reads an [`Env`] from the provided reader
    ///
    /// Returns a [`DeserializationError`] if the reading operation fails.
    pub fn from_reader<R>(reader: R) -> Result<Env, DeserializationError>
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
        Env,
        de::{from_reader, from_string},
        ser::{to_string, to_writer},
    };

    #[test]
    fn serialize_env_file_to_string() {
        //* Given
        let env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

        //* When
        let serialized = to_string(&env_file).expect("failed to serialize env file");

        //* Then
        let expected = "KEY1=\"VALUE1\"\nKEY2=\"VALUE2\"";
        assert_eq!(serialized, expected);
    }

    #[test]
    fn serialize_env_file_to_writer() {
        //* Given
        let env_file = Env::from_iter([("KEY1", "VALUE1"), ("KEY2", "VALUE2")]);

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
        let expected_env_file = Env::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);
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
        let expected_env_file = Env::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);
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
        let original_env_file = Env::from_iter([("key1", "VALUE1"), ("key2", "VALUE2")]);

        //* When
        let serialized = to_string(&original_env_file).expect("Failed to serialize env file");
        let deserialized_env_file =
            from_string(&serialized).expect("Failed to deserialize env file");

        //* Then
        assert_eq!(original_env_file, deserialized_env_file);
    }
}
