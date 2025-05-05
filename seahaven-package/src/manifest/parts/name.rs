/// A name that is validated against the `^[a-zA-Z0-9._-]+$` regex.
#[derive(Debug, Clone, Hash, serde::Serialize)]
#[serde(transparent)]
pub struct Name(String);

impl Name {
    /// Consumes the [`Name`] and returns the inner [`String`].
    ///
    /// The returned [`String`] is guaranteed to be valid.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::borrow::Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> std::cmp::PartialEq<T> for Name
where
    T: ?Sized + AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.as_str() == other.as_ref()
    }
}

impl std::cmp::Eq for Name {}

impl std::str::FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.to_string();
        if !parse::is_valid_name(&name) {
            return Err(Error { name });
        }
        Ok(Name(name))
    }
}

/// An error that occurs when a name is invalid.
#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid name: '{name}'")]
pub struct Error {
    pub name: String,
}

mod parse {
    use once_cell::sync::Lazy;
    use serde::de::Error as _;

    use super::{Error, Name};

    impl Name {
        /// Creates a new name from a [`String`].
        ///
        /// This is an internal function that does not validate the name.
        fn new_unchecked(name: impl Into<String>) -> Self {
            Self(name.into())
        }
    }

    impl<'de> serde::Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let name = String::deserialize(deserializer)?;
            if !is_valid_name(&name) {
                return Err(D::Error::custom(Error { name }));
            }
            Ok(Name::new_unchecked(name))
        }
    }

    /// Validates that a name is valid.
    ///
    /// A name is valid if it matches the `^[a-zA-Z0-9._-]+$` regex.
    pub fn is_valid_name(name: &str) -> bool {
        static NAME_REGEX: Lazy<regress::Regex> =
            Lazy::new(|| regress::Regex::new(r#"^[a-zA-Z0-9._-]+$"#).expect("Invalid regex"));

        NAME_REGEX.find(name).is_some()
    }
}

mod display {
    use super::Name;

    impl std::fmt::Display for Name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Name, parse};

    const VALID_NAME: &str = "my-name";
    const INVALID_NAME: &str = "my/name";

    #[test]
    fn valid_names() {
        assert!(parse::is_valid_name("my-name"));
        assert!(parse::is_valid_name("my_name"));
        assert!(parse::is_valid_name("my.name"));
        assert!(parse::is_valid_name("my-name"));
    }

    #[test]
    fn invalid_names() {
        assert!(!parse::is_valid_name("my name"));
        assert!(!parse::is_valid_name("my/name"));
    }

    #[test]
    fn valid_from_str() {
        //* Given
        let name = VALID_NAME;

        //* When
        let result = name.parse::<Name>().expect("Invalid name");

        //* Then
        assert_eq!(result.into_inner(), name);
    }

    #[test]
    fn invalid_from_str() {
        //* Given
        let name = INVALID_NAME;

        //* When
        let result = name.parse::<Name>();

        //* Then
        assert!(result.is_err(), "Expected error for invalid name: '{name}'",);

        let error = result.expect_err("Expected error");
        assert!(
            error.to_string().contains("invalid name"),
            "Expected error to contain 'invalid name'"
        );
        assert!(
            error.to_string().contains(name),
            "Expected error to contain the invalid name"
        );
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    struct NamedThing {
        name: Name,
    }

    #[test]
    fn valid_name_deserialization() {
        //* Given
        let name = VALID_NAME;

        let toml_string = format!(r#"name = "{name}""#);

        //* When
        let result =
            toml::from_str::<NamedThing>(&toml_string).expect("Failed to deserialize name");

        //* Then
        assert_eq!(result.name, name);
    }

    #[test]
    fn invalid_name_deserialization() {
        //* Given
        let name = INVALID_NAME;

        let toml_string = format!(r#"name = "{name}""#);

        //* When
        let result = toml::from_str::<NamedThing>(&toml_string);

        //* Then
        assert!(result.is_err(), "Expected error for invalid name: '{name}'",);

        let error = result.expect_err("Expected error");
        assert!(
            error.to_string().contains("invalid name"),
            "Expected error to contain 'invalid name'"
        );
        assert!(
            error.to_string().contains(name),
            "Expected error to contain the invalid name"
        );
    }
}
