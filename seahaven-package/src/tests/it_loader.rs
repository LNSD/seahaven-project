use std::path::PathBuf;

use super::common::TESTS_DIR_PATH;
use crate::loader::{Error, FileLoader, Loader};

#[test]
fn load_manifest_from_explicit_toml_file() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH).join("fixtures");
    let path = PathBuf::from("contracts/package.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let manifest = loader.load(&path).expect("Failed to load manifest");

    //* Then
    assert_eq!(manifest.package.name, "kitchen-sink");
}

#[test]
fn load_manifest_from_directory_defaults_to_package_toml() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH).join("fixtures");
    let package_path = PathBuf::from("contracts/");

    let loader = FileLoader::new(root_path);

    //* When
    let manifest = loader.load(&package_path).expect("Failed to load manifest");

    //* Then
    assert_eq!(manifest.package.name, "kitchen-sink");
}

#[test]
fn load_manifest_fails_when_file_not_found() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH).join("fixtures");
    let package_path = PathBuf::from("non_existent.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&package_path);

    //* Then
    let err = result.expect_err("Should return an error when file does not exist");
    assert!(matches!(err, Error::FileOpen { .. }));
}

#[test]
fn load_manifest_fails_when_dir_path_does_not_exist() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH).join("fixtures");
    let package_path = PathBuf::from("non_existent_directory/");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&package_path);

    //* Then
    let err = result.expect_err("Should return an error when path does not exist");
    assert!(matches!(err, Error::FileOpen { .. }));
}

#[test]
fn load_manifest_fails_when_directory_has_no_package_toml() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH);
    let package_path = PathBuf::from("fixtures");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&package_path);

    //* Then
    let err = result.expect_err("Should return an error when package.toml is missing");
    assert!(matches!(err, Error::FileOpen { .. }));
}

#[test]
fn load_manifest_fails_when_root_path_invalid() {
    //* Given
    let root_path = PathBuf::from("/non/existent/path/");
    let package_path = PathBuf::from("kitchen_sink.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&package_path);

    //* Then
    let err = result.expect_err("Should return an error when file path is invalid");
    assert!(matches!(err, Error::FileOpen { .. }));
}

#[test]
fn load_manifest_fails_when_file_is_not_a_valid_manifest() {
    //* Given
    let root_path = PathBuf::from(TESTS_DIR_PATH).join("fixtures");
    let package_path = PathBuf::from("invalid_package.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&package_path);

    //* Then
    let err = result.expect_err("Should return an error when file is not a valid manifest");
    assert!(matches!(err, Error::ManifestParse { .. }));
}
