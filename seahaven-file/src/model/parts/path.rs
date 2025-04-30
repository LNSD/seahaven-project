use std::path::{Path as StdPath, PathBuf};

use serde::ser::Error as _;

/// A relative file path.
///
/// This [`Path`] type represents a relative path with strict validation rules:
/// - Paths must be non-empty
/// - Paths must be relative to the current working directory
/// - Paths cannot contain path traversal components (e.g., `..` or `//`)
/// - Paths may only start with `.` if it represents the current directory
///
/// The [`Path`] type implements serialization and deserialization,
/// ensuring that paths remain valid when loaded from configuration files.
///
/// # Safety
///
/// The [`Path`] type enforces strict validation to prevent path traversal attacks and
/// ensure that all paths are relative to the current working directory.
#[derive(Clone, PartialEq, Eq)]
pub struct Path(Box<StdPath>);

impl Path {
    /// Create a new [`Path`] from a string.
    #[cfg(test)]
    fn new_unchecked(path: impl Into<String>) -> Self {
        Self(std::path::PathBuf::from(path.into()).into_boxed_path())
    }

    /// Consume the [`Path`] and return the inner boxed path.
    pub fn into_inner(self) -> Box<StdPath> {
        self.0
    }

    /// Returns an object that implements [`Display`] for safely printing paths that
    /// may contain non-Unicode data. This may perform lossy conversion, depending on the platform.
    ///
    /// If you would like an implementation which escapes the path please use [`Debug`] instead.
    ///
    /// [`Display`]: std::fmt::Display
    /// [`Debug`]: std::fmt::Debug
    pub fn display(&self) -> impl std::fmt::Display {
        self.0.display()
    }
}

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        &self.0
    }
}

impl std::ops::Deref for Path {
    type Target = StdPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Path MUST NOT be empty
        if s.is_empty() {
            return Err(Error {
                reason: "empty path".to_string(),
                path: s.to_string(),
            });
        }

        let path = PathBuf::from(s).into_boxed_path();

        // It MUST be a relative path, i.e. it MUST NOT contain any path traversal components,
        // It MUST NOT contain any path traversal components, only an starting `.` is allowed.
        let mut components = path.components().peekable();
        if matches!(components.peek(), Some(std::path::Component::CurDir)) {
            components.next();
        }

        // Check if the path has any prefix and/or is absolute
        if components.clone().any(|c: std::path::Component| {
            matches!(
                c,
                std::path::Component::Prefix(_) | std::path::Component::RootDir
            )
        }) {
            return Err(Error {
                reason: "cannot contain prefix or absolute path".to_string(),
                path: s.to_string(),
            });
        }

        // Check if the path contains any path traversal components
        if components
            .clone()
            .any(|c: std::path::Component| matches!(c, std::path::Component::ParentDir))
        {
            return Err(Error {
                reason: "cannot contain path traversal components".to_string(),
                path: s.to_string(),
            });
        }

        Ok(Path(path))
    }
}

/// An error that occurs when a path is invalid.
#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid path '{path}': {reason}")]
pub struct Error {
    pub reason: String,
    pub path: String,
}

impl<'de> serde::de::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        s.parse::<Path>().map_err(serde::de::Error::custom)
    }
}

impl serde::ser::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path_str = self
            .0
            .to_str()
            .ok_or(S::Error::custom("path must be valid UTF-8"))?;
        serializer.serialize_str(path_str)
    }
}

#[cfg(test)]
mod tests {
    use super::Path;

    #[test]
    fn valid_relative_path() {
        //* Given
        let path_str = "config/app.yaml";

        //* When
        let path = path_str.parse::<Path>().expect("path should be valid");

        //* Then
        assert_eq!(path.to_string_lossy(), path_str);
    }

    #[test]
    fn empty_path_error() {
        //* Given
        let path_str = "";

        //* When
        let err = path_str
            .parse::<Path>()
            .expect_err("path should be invalid");

        //* Then
        assert_eq!(err.reason, "empty path");
        assert_eq!(err.path, path_str);
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn absolute_path_error() {
        //* Given
        let path_str = "/etc/config.yaml";

        //* When
        let err = path_str
            .parse::<Path>()
            .expect_err("path should be invalid");

        //* Then
        assert_eq!(err.reason, "cannot contain prefix or absolute path");
        assert_eq!(err.path, path_str);
    }

    #[test]
    fn path_traversal_error() {
        //* Given
        let path_str = "../config.yaml";

        //* When
        let err = path_str
            .parse::<Path>()
            .expect_err("path should be invalid");

        //* Then
        assert_eq!(err.reason, "cannot contain path traversal components");
        assert_eq!(err.path, path_str);
    }

    #[test]
    fn current_dir_allowed() {
        //* Given
        let path_str = "./config.yaml";

        //* When
        let path = path_str.parse::<Path>().expect("path should be valid");

        //* Then
        assert_eq!(path.to_string_lossy(), path_str);
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn windows_absolute_path_error() {
        //* Given
        let path_str = "C:\\config.yaml";

        //* When
        let err = path_str
            .parse::<Path>()
            .expect_err("path should be invalid");

        //* Then
        assert_eq!(err.reason, "cannot contain prefix or absolute path");
        assert_eq!(err.path, path_str);
    }

    #[test]
    fn serialize_path() {
        //* Given
        let path = Path::new_unchecked("config/app.yaml");

        //* When
        let serialized = serde_yaml::to_string(&path).expect("path should be serializable");

        //* Then
        assert_eq!(serialized.trim(), "config/app.yaml");
    }

    #[test]
    fn deserialize_path() {
        //* Given
        let path_str = "config/app.yaml";

        //* When
        let path = serde_yaml::from_str::<Path>(path_str).expect("path should be deserializable");

        //* Then
        assert_eq!(path.to_string_lossy(), path_str);
    }
}
