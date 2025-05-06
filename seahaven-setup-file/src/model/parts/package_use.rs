use super::{
    name::{Error as NameError, Name},
    path::{Error as PathError, Path},
};

/// A package `use` entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct PackageUse {
    /// The path to the package
    pub path: Path,

    /// The target to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Name>,
}

impl std::str::FromStr for PackageUse {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the use string into path and target
        let (path, target) = match s.split_once('#') {
            Some((path, target)) => (path, Some(target)),
            None => (s, None),
        };

        // Parse the path
        let path = path.parse::<Path>()?;

        // Parse the target, and return an error if it's an invalid name
        let target = target.map(|target| target.parse::<Name>()).transpose()?;

        Ok(PackageUse { path, target })
    }
}

/// An error that occurs when a package `use` is invalid.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("invalid package path '{path}': {reason}")]
    InvalidPath { reason: String, path: String },

    #[error("invalid package target: '{name}'")]
    InvalidTarget { name: String },
}

impl From<NameError> for Error {
    fn from(NameError { name }: NameError) -> Self {
        Error::InvalidTarget { name }
    }
}

impl From<PathError> for Error {
    fn from(PathError { reason, path }: PathError) -> Self {
        Error::InvalidPath { reason, path }
    }
}

#[cfg(test)]
mod tests {
    use super::PackageUse;
    use crate::model::parts::common::FromStructOrString;

    /// A test struct that deserializes a package use from a string or a struct.
    #[serde_with::serde_as]
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct TestStruct {
        #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
        #[serde_as(as = "Option<FromStructOrString>")]
        pub package_use: Option<PackageUse>,
    }

    #[test]
    fn deserialize_from_string() {
        //* Given
        let input = r#"use: path/to/package#target"#;

        //* When
        let result = serde_yaml::from_str::<TestStruct>(input).expect("deserialization failed");

        //* Then
        let TestStruct { package_use } = result;

        let package_use = package_use.expect("package use should be Some");

        let path = package_use
            .path
            .to_str()
            .expect("failed to convert path to string");
        assert_eq!(path, "path/to/package");

        let target = package_use.target.expect("target is not set");
        assert_eq!(target, "target");
    }

    #[test]
    fn deserialize_from_string_no_target() {
        //* Given
        let input = r#"use: path/to/package"#;

        //* When
        let result = serde_yaml::from_str::<TestStruct>(input).expect("deserialization failed");

        //* Then
        let TestStruct { package_use } = result;

        let package_use = package_use.expect("package use should be Some");

        let path = package_use
            .path
            .to_str()
            .expect("failed to convert path to string");
        assert_eq!(path, "path/to/package");
        assert!(package_use.target.is_none());
    }

    #[test]
    fn deserialize_from_yaml_mapping() {
        //* Given
        let input = indoc::indoc! {r#"
            use:
              path: path/to/package
              target: target
        "#};

        //* When
        let result = serde_yaml::from_str::<TestStruct>(input).expect("deserialization failed");

        //* Then
        let TestStruct { package_use } = result;

        let package_use = package_use.expect("package use should be Some");

        let path = package_use
            .path
            .to_str()
            .expect("failed to convert path to string");
        assert_eq!(path, "path/to/package");

        let target = package_use.target.expect("target is not set");
        assert_eq!(target, "target");
    }

    #[test]
    fn deserialize_from_yaml_mapping_no_target() {
        //* Given
        let input = indoc::indoc! {r#"
            use:
              path: path/to/package
        "#};

        //* When
        let result = serde_yaml::from_str::<TestStruct>(input).expect("deserialization failed");

        //* Then
        let TestStruct { package_use } = result;

        let package_use = package_use.expect("package use should be Some");

        let path = package_use
            .path
            .to_str()
            .expect("failed to convert path to string");
        assert_eq!(path, "path/to/package");
        assert!(package_use.target.is_none());
    }

    #[test]
    fn deserialize_from_empty_string() {
        //* Given
        let input = r#"use: """#;

        //* When
        let result =
            serde_yaml::from_str::<TestStruct>(input).expect_err("deserialization should fail");

        //* Then
        assert!(result.to_string().contains("invalid package path"));
        assert!(result.to_string().contains("empty path"));
    }

    #[test]
    fn deserialize_from_number() {
        //* Given
        let input = r#"use: 123"#;

        //* When
        let result =
            serde_yaml::from_str::<TestStruct>(input).expect_err("deserialization should fail");

        //* Then
        assert!(result.to_string().contains("invalid type"));
        assert!(result.to_string().contains("expected string or map"));
    }

    #[test]
    fn deserialize_from_yaml_mapping_with_empty_path() {
        //* Given
        let input = indoc::indoc! {r#"
            use:
              path: ""
        "#};

        //* When
        let result =
            serde_yaml::from_str::<TestStruct>(input).expect_err("deserialization should fail");

        //* Then
        assert!(result.to_string().contains("invalid path"));
        assert!(result.to_string().contains("empty path"));
    }

    #[test]
    fn deserialize_from_invalid_target_name() {
        //* Given
        let input = r#"use: path/to/package#invalid/target"#;

        //* When
        let result =
            serde_yaml::from_str::<TestStruct>(input).expect_err("deserialization should fail");

        //* Then
        assert!(result.to_string().contains("invalid package target"));
        assert!(result.to_string().contains("invalid/target"));
    }

    #[test]
    fn serialize_to_yaml_mapping() {
        //* Given
        let test_struct = TestStruct {
            package_use: Some(
                "path/to/package#target"
                    .parse::<PackageUse>()
                    .expect("failed to parse package use"),
            ),
        };

        let expected = indoc::indoc! {r#"
            use:
              path: path/to/package
              target: target
        "#};

        //* When
        let result = serde_yaml::to_string(&test_struct).expect("serialization failed");

        //* Then
        assert_eq!(result, expected);
    }
}
