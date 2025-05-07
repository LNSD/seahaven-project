use std::path::PathBuf;

use super::common::FIXTURES_DIR_PATH;
use crate::loader::{FileLoader, Loader};

#[test]
fn load_package_from_file() {
    //* Given
    let root_path = PathBuf::from(FIXTURES_DIR_PATH);
    let fixture_path = PathBuf::from("kitchen_sink.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let manifest = loader.load(&fixture_path).expect("Failed to load manifest");

    //* Then
    assert_eq!(manifest.package.name, "kitchen-sink");
}

#[test]
fn load_package_from_non_existent_file() {
    //* Given
    let root_path = PathBuf::from(FIXTURES_DIR_PATH);
    let fixture_path = PathBuf::from("non_existent.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&fixture_path);

    //* Then
    assert!(result.is_err(), "Should return an error");
}

#[test]
fn load_package_from_wrong_root_path() {
    //* Given
    let root_path = PathBuf::from("/a/non/existent/path/");
    let fixture_path = PathBuf::from("kitchen_sink.toml");

    let loader = FileLoader::new(root_path);

    //* When
    let result = loader.load(&fixture_path);

    //* Then
    assert!(result.is_err(), "Should return an error");
}
