//! # Seahaven setup description file model
//!
//! This module provides types and functions for working with Seahaven setup description files.
//! It includes the [`Content`] struct that represents the content of a setup file,
//! along with serialization and deserialization utilities.

mod parts;

pub mod display;
pub mod parsing;

pub use parts::{ValidatedRoot as Content, *};

#[cfg(test)]
mod tests {
    use seahaven_file_testdata::SINGLE_SERVICE;

    use super::{
        display::{to_string, to_writer},
        parsing::{from_reader, from_str},
    };

    #[test]
    fn deserialize_setup_file_from_str() {
        //* Given
        let setup_file = SINGLE_SERVICE;

        //* When
        let file = from_str(setup_file).expect("Failed to deserialize setup file");

        //* Then
        assert!(file.name.is_none());
        assert_eq!(file.services.len(), 1);
        assert!(file.init.is_none());
        assert!(file._rest.is_empty());
    }

    #[test]
    fn deserialize_setup_file_from_reader() {
        //* Given
        let reader = SINGLE_SERVICE.as_bytes();

        //* When
        let file = from_reader(reader).expect("Failed to deserialize setup file");

        //* Then
        assert!(file.name.is_none());
        assert_eq!(file.services.len(), 1);
        assert!(file.init.is_none());
        assert!(file._rest.is_empty());
    }

    #[test]
    fn serialize_setup_file_to_string() {
        //* Given
        let setup_file = from_str(SINGLE_SERVICE).expect("Failed to deserialize setup file");

        //* When
        let serialized = to_string(&setup_file).expect("Failed to serialize setup file");

        //* Then
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], SINGLE_SERVICE);
    }

    #[test]
    fn serialize_setup_file_to_writer() {
        //* Given
        let setup_file = from_str(SINGLE_SERVICE).expect("Failed to deserialize setup file");

        let mut writer = Vec::new();

        //* When
        to_writer(&mut writer, &setup_file).expect("Failed to serialize setup file");

        //* Then
        let serialized =
            String::from_utf8(writer).expect("Failed to convert serialized bytes to string");
        // The last character of the serialized string is a newline, so we need to remove it
        assert_eq!(&serialized[..serialized.len() - 1], SINGLE_SERVICE);
    }
}
