use std::io::Cursor;

use testlib_file_testdata::setup_yaml::{
    EMPTY_ENV, KITCHEN_SINK, NO_SERVICES, SINGLE_SERVICE, SINGLE_SERVICE_BASIC,
};

use crate::{ParsingError, envfile_from_reader, from_reader};

#[test]
fn single_service() {
    // * Given
    let input = Cursor::new(SINGLE_SERVICE);

    // * When
    let (env_file, content) = from_reader(input).expect("Failed to parse content");

    // * Then
    // Verify envfile is not present
    assert!(env_file.is_none());

    // Verify service content at top level only
    assert!(!content.services.is_empty());
    assert!(content.services.contains_key("app"));
}

#[test]
fn single_service_basic() {
    // * Given
    let input = Cursor::new(SINGLE_SERVICE_BASIC);

    // * When
    let (env_file, content) = from_reader(input).expect("Failed to parse content");

    // * Then
    // Verify envfile is not present
    assert!(env_file.is_none());

    // Verify service content at top level only
    assert!(!content.services.is_empty());
    assert!(content.services.contains_key("app"));
}

#[test]
fn no_services_error() {
    // * Given
    let input = Cursor::new(NO_SERVICES);

    // * When
    let result = from_reader(input);

    // * Then
    assert!(
        matches!(result, Err(ParsingError::ContentDeserializationFailed(_))),
        "Expected ContentDeserializationFailed error, got {:?}",
        result
    );
}

#[test]
fn empty_env() {
    // * Given
    let input = Cursor::new(EMPTY_ENV);

    // * When
    let (env_file, content) = from_reader(input).expect("Failed to parse EMPTY_ENV");

    // * Then
    // Verify env file is present but empty
    assert!(
        matches!(env_file, Some(env_file) if env_file.is_empty()),
        "Expected env file to be present but empty"
    );

    // Verify the top-level content
    assert!(!content.services.is_empty());
    assert!(content.volumes.is_some());
}

#[test]
fn kitchen_sink() {
    // * Given
    let input = Cursor::new(KITCHEN_SINK);

    // * When
    let (env_file, content) = from_reader(input).expect("Failed to parse KITCHEN_SINK");

    // * Then
    // Assert that the env file is present and has the correct values
    let env_file = env_file.expect("Front matter should be present");
    assert_eq!(env_file.get("chain_id").unwrap(), "1337");
    assert_eq!(env_file.get("chain_name").unwrap(), "hardhat");
    assert_eq!(env_file.get("app_port").unwrap(), "8080");
    assert_eq!(env_file.get("chain_rpc").unwrap(), "8545");
    assert_eq!(env_file.get("db_password").unwrap(), "secret");
    assert_eq!(env_file.get("app_server_admin").unwrap(), "7600");
    assert_eq!(env_file.get("app_server_rpc").unwrap(), "7601");
    assert_eq!(env_file.get("app_server_metrics").unwrap(), "7602");

    // Assert that the content structure is correct
    assert!(content.name.is_some());
    assert_eq!(content.services.len(), 3);
    assert!(content.networks.is_some());
    assert!(content.volumes.is_some());
    assert!(content.configs.is_some());
    assert!(content.secrets.is_some());
}

#[test]
fn kitchen_sink_env_file() {
    // * Given
    let input = Cursor::new(KITCHEN_SINK);

    // * When
    let env_file = envfile_from_reader(input).expect("Failed to parse KITCHEN_SINK");

    // * Then
    let env_file = env_file.expect("Env file should be present");
    assert_eq!(env_file.get("chain_id").unwrap(), "1337");
    assert_eq!(env_file.get("chain_name").unwrap(), "hardhat");
    assert_eq!(env_file.get("app_port").unwrap(), "8080");
    assert_eq!(env_file.get("chain_rpc").unwrap(), "8545");
    assert_eq!(env_file.get("db_password").unwrap(), "secret");
    assert_eq!(env_file.get("app_server_admin").unwrap(), "7600");
    assert_eq!(env_file.get("app_server_rpc").unwrap(), "7601");
    assert_eq!(env_file.get("app_server_metrics").unwrap(), "7602");
}
