use std::io::Cursor;

use seahaven_setup_file_testdata::{
    EMPTY_ENV, KITCHEN_SINK, NO_SERVICES, PACKAGE_USE, PACKAGE_USE_NO_TARGET_INIT, SINGLE_SERVICE,
    SINGLE_SERVICE_BASIC,
};

use crate::parsing::{ParsingError, env_from_reader, from_reader};

#[test]
fn single_service() {
    // * Given
    let input = Cursor::new(SINGLE_SERVICE);

    // * When
    let setup_file = from_reader(input).expect("Failed to parse content");

    // * Then
    // Verify envfile is not present
    let env = setup_file.env();
    assert!(env.is_none());

    // Verify service content at top level only
    let content = setup_file.content();
    assert!(!content.services.is_empty());
    assert!(content.services.contains_key("app"));
}

#[test]
fn single_service_basic() {
    // * Given
    let input = Cursor::new(SINGLE_SERVICE_BASIC);

    // * When
    let setup_file = from_reader(input).expect("Failed to parse content");

    // * Then
    // Verify envfile is not present
    let env = setup_file.env();
    assert!(env.is_none());

    // Verify service content at top level only
    let content = setup_file.content();
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
        matches!(result, Err(ParsingError::ContentParsingFailed(_))),
        "Expected ContentDeserializationFailed error, got {result:?}"
    );
}

#[test]
fn empty_env() {
    // * Given
    let input = Cursor::new(EMPTY_ENV);

    // * When
    let setup_file = from_reader(input).expect("Failed to parse EMPTY_ENV");

    // * Then
    // Verify env file is present but empty
    assert!(
        matches!(setup_file.env(), Some(env_file) if env_file.is_empty()),
        "Expected env file to be present but empty"
    );

    // Verify the top-level content
    let content = setup_file.content();
    assert!(!content.services.is_empty());
    assert!(!content._rest.is_empty());
}

#[test]
fn kitchen_sink() {
    // * Given
    let input = Cursor::new(KITCHEN_SINK);

    // * When
    let setup_file = from_reader(input).expect("Failed to parse KITCHEN_SINK");

    // * Then
    // Assert that the env file is present and has the correct values
    let env = setup_file.env().expect("Front matter should be present");
    assert_eq!(env.get("chain_id").unwrap(), "1337");
    assert_eq!(env.get("chain_name").unwrap(), "hardhat");
    assert_eq!(env.get("app_port").unwrap(), "8080");
    assert_eq!(env.get("chain_rpc").unwrap(), "8545");
    assert_eq!(env.get("db_password").unwrap(), "secret");
    assert_eq!(env.get("app_server_admin").unwrap(), "7600");
    assert_eq!(env.get("app_server_rpc").unwrap(), "7601");
    assert_eq!(env.get("app_server_metrics").unwrap(), "7602");

    // Assert that the content structure is correct
    let content = setup_file.content();
    assert!(content.name.is_some());
    assert_eq!(content.services.len(), 3);
    assert!(content.init.is_some());
    assert!(!content._rest.is_empty());
}

#[test]
fn kitchen_sink_env_file() {
    // * Given
    let input = Cursor::new(KITCHEN_SINK);

    // * When
    let env = env_from_reader(input).expect("Failed to parse KITCHEN_SINK");

    // * Then
    let env = env.expect("Env file should be present");
    assert_eq!(env.get("chain_id").unwrap(), "1337");
    assert_eq!(env.get("chain_name").unwrap(), "hardhat");
    assert_eq!(env.get("app_port").unwrap(), "8080");
    assert_eq!(env.get("chain_rpc").unwrap(), "8545");
    assert_eq!(env.get("db_password").unwrap(), "secret");
    assert_eq!(env.get("app_server_admin").unwrap(), "7600");
    assert_eq!(env.get("app_server_rpc").unwrap(), "7601");
    assert_eq!(env.get("app_server_metrics").unwrap(), "7602");
}

#[test]
fn package_use() {
    // * Given
    let input = Cursor::new(PACKAGE_USE);

    // * When
    let setup_file = from_reader(input).expect("Failed to parse PACKAGE_USE");

    // * Then
    let init = setup_file
        .content()
        .init
        .as_ref()
        .expect("Init should be present");
    let deploy_contracts = init
        .get("deploy-contracts")
        .expect("Deploy contracts should be present");
    assert!(deploy_contracts.package_use.is_some());

    let package_use = deploy_contracts
        .package_use
        .as_ref()
        .expect("Package use should be present");

    assert_eq!(package_use.target.as_ref().unwrap(), "deploy");
}

#[test]
fn package_use_no_target_init_error() {
    // * Given
    let input = Cursor::new(PACKAGE_USE_NO_TARGET_INIT);

    // * When
    let result = from_reader(input);

    // * Then
    println!("result: {result:?}");
    assert!(
        matches!(result, Err(ParsingError::ContentParsingFailed(_))),
        "Expected ContentDeserializationFailed error, got {result:?}"
    );
}
