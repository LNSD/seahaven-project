use std::fs;

use seahaven_compose_file::ComposeFile;
use seahaven_setup_file::Env;

use crate::files::tempdir::{self, HasComposeFilePath, HasEnvFilePath, HasTempDirPath};

/// Creates a test environment file
fn test_env_file() -> Env {
    let mut env = Env::new();
    env.insert("TEST_KEY".to_string(), "test_value".to_string());
    env.insert("ANOTHER_KEY".to_string(), "another_value".to_string());
    env
}

/// Creates a test compose file
fn test_compose_file() -> ComposeFile {
    let services = serde_yaml::Mapping::from_iter([(
        serde_yaml::Value::String("test-service".to_string()),
        serde_yaml::Value::Mapping(serde_yaml::Mapping::from_iter([(
            serde_yaml::Value::String("image".to_string()),
            serde_yaml::Value::String("test-image:latest".to_string()),
        )])),
    )]);

    ComposeFile {
        name: Some("test-project".to_string()),
        services,
        networks: None,
        volumes: None,
        configs: None,
        secrets: None,
    }
}

#[test]
fn creates_temp_dir() {
    //* Given
    let uninit = tempdir::new();

    //* When
    let dir_created = uninit.create_dir().expect("Failed to create temp dir");

    //* Then
    assert!(
        dir_created.temp_dir_path().is_dir(),
        "Temp dir should exist"
    );
}

#[test]
fn writes_env_file_with_contents() {
    //* Given
    let env_file = test_env_file();

    let created_dir = tempdir::new()
        .create_dir()
        .expect("Failed to create temp dir");

    //* When
    let env_file_only = created_dir
        .write_env_file(&env_file)
        .expect("Failed to write env file");

    //* Then
    assert!(
        env_file_only.temp_dir_path().is_dir(),
        "Temp dir should exist"
    );

    // Assert env file was correctly written
    assert!(
        env_file_only.env_file_path().is_file(),
        "Env file should exist"
    );
    let contents =
        fs::read_to_string(env_file_only.env_file_path()).expect("Failed to read env file");
    assert!(!contents.is_empty(), "Env file should not be empty");
}

#[test]
fn writes_all_files_successfully() {
    //* Given
    let env_file = test_env_file();
    let compose_file = test_compose_file();

    let created_dir = tempdir::new()
        .create_dir()
        .expect("Failed to create temp dir");

    //* When
    let complete = created_dir
        .write_all(&env_file, &compose_file)
        .expect("Failed to write all files");

    //* Then
    assert!(complete.temp_dir_path().is_dir(), "Temp dir should exist");

    // Assert env file was correctly written
    assert!(complete.env_file_path().is_file(), "Env file should exist");
    let env_contents =
        fs::read_to_string(complete.env_file_path()).expect("Failed to read env file");
    assert!(!env_contents.is_empty(), "Env file should not be empty");

    // Assert compose file was correctly written
    assert!(
        complete.compose_file_path().is_file(),
        "Compose file should exist"
    );
    let compose_file_contents =
        fs::read_to_string(complete.compose_file_path()).expect("Failed to read compose file");
    assert!(
        !compose_file_contents.is_empty(),
        "Compose file should not be empty"
    );
}

#[test]
fn cleans_up_temp_dir_on_drop() {
    //* Given
    let test_env_file = test_env_file();
    let test_compose_file = test_compose_file();

    let created_dir = tempdir::new()
        .create_dir()
        .expect("Failed to create temp dir");

    let complete = created_dir
        .write_all(&test_env_file, &test_compose_file)
        .expect("Failed to write all files");

    //* When
    // Track the paths to check if they exist after the complete struct is dropped
    let temp_dir_path = complete.temp_dir_path().to_path_buf();
    let env_file_path = complete.env_file_path().to_path_buf();
    let compose_file_path = complete.compose_file_path().to_path_buf();

    // Drop the complete struct
    drop(complete);

    //* Then
    assert!(!temp_dir_path.is_dir(), "Temp dir should not exist");
    assert!(!env_file_path.is_file(), "Env file should not exist");
    assert!(
        !compose_file_path.is_file(),
        "Compose file should not exist"
    );
}
