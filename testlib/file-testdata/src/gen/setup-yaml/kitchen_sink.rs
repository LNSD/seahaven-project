/// Test vector: `setup-yaml/kitchen_sink`
///
/// ```yaml
/// # The following YAML file is a "kitchen sink" of sorts, containing a
/// # variety of features and edge cases that are commonly encountered in
/// # the Seahaven setup description file.
/// #
/// # What is exact meaning of "kitchen sink" in programming?
/// # https://stackoverflow.com/q/33779296/1099999
///
/// ---
/// # Chain config
/// CHAIN_RPC: 8545
/// CHAIN_ID: 1337
/// CHAIN_NAME: "hardhat"
///
/// # App server
/// APP_SERVER_ADMIN: 7600
/// APP_SERVER_RPC: 7601
/// APP_SERVER_METRICS: 7602
/// ---
///
/// services:
///   chain:
///     image: ghcr.io/foundry-rs/foundry:latest
///     command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
///     ports:
///       - "${CHAIN_RPC}:8545"
///     healthcheck:
///       { interval: 1s, retries: 10, test: cast block }
///
///   app-server:
///     image: ghcr.io/example/server:latest
///     depends_on:
///       chain: { condition: service_healthy }
///       deploy-contracts: { condition: service_completed_successfully }
///     ports:
///       - "${APP_SERVER_ADMIN}:7600"
///       - "${APP_SERVER_RPC}:7601"
///       - "${APP_SERVER_METRICS}:7602"
///     healthcheck:
///       { interval: 1s, retries: 10, test: curl -sf http://localhost:${APP_SERVER_ADMIN}/health }
///
/// init-containers:
///   deploy-contracts:
///     build: { context: contracts }
///     depends_on:
///       chain: { condition: service_healthy }
///     volumes:
///       - ./contracts.json:/opt/contracts.json:ro
/// ```
///
/// See file: `setup-yaml/kitchen_sink.yaml`
pub const KITCHEN_SINK: &str = indoc::indoc! { r###"
  # The following YAML file is a "kitchen sink" of sorts, containing a
  # variety of features and edge cases that are commonly encountered in
  # the Seahaven setup description file.
  #
  # What is exact meaning of "kitchen sink" in programming?
  # https://stackoverflow.com/q/33779296/1099999

  ---
  # Chain config
  CHAIN_RPC: 8545
  CHAIN_ID: 1337
  CHAIN_NAME: "hardhat"

  # App server
  APP_SERVER_ADMIN: 7600
  APP_SERVER_RPC: 7601
  APP_SERVER_METRICS: 7602
  ---

  services:
    chain:
      image: ghcr.io/foundry-rs/foundry:latest
      command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
      ports:
        - "${CHAIN_RPC}:8545"
      healthcheck:
        { interval: 1s, retries: 10, test: cast block }

    app-server:
      image: ghcr.io/example/server:latest
      depends_on:
        chain: { condition: service_healthy }
        deploy-contracts: { condition: service_completed_successfully }
      ports:
        - "${APP_SERVER_ADMIN}:7600"
        - "${APP_SERVER_RPC}:7601"
        - "${APP_SERVER_METRICS}:7602"
      healthcheck:
        { interval: 1s, retries: 10, test: curl -sf http://localhost:${APP_SERVER_ADMIN}/health }

  init-containers:
    deploy-contracts:
      build: { context: contracts }
      depends_on:
        chain: { condition: service_healthy }
      volumes:
        - ./contracts.json:/opt/contracts.json:ro"### };
