#[doc = "Test vector: `seahaven-file/testdata/data/package_use_no_target_init`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"---"##]
#[doc = r##"# Chain config"##]
#[doc = r##"CHAIN_RPC = 8545"##]
#[doc = r##"CHAIN_ID = 1337"##]
#[doc = r##"CHAIN_NAME = "hardhat""##]
#[doc = r##""##]
#[doc = r##"# App server"##]
#[doc = r##"APP_SERVER_ADMIN = 7600"##]
#[doc = r##"APP_SERVER_RPC = 7601"##]
#[doc = r##"APP_SERVER_METRICS = 7602"##]
#[doc = r##"---"##]
#[doc = r##""##]
#[doc = r##"services:"##]
#[doc = r##"  chain:"##]
#[doc = r##"    use: ./chain"##]
#[doc = r##"    environment:"##]
#[doc = r##"      CHAIN_RPC: "${CHAIN_RPC}""##]
#[doc = r##""##]
#[doc = r##"  app-server:"##]
#[doc = r##"    image: ghcr.io/example/server:latest"##]
#[doc = r##"    depends_on:"##]
#[doc = r##"      chain: { condition: service_healthy }"##]
#[doc = r##"      deploy-contracts: { condition: service_completed_successfully }"##]
#[doc = r##"    ports:"##]
#[doc = r##"      - "${APP_SERVER_ADMIN}:7600""##]
#[doc = r##"      - "${APP_SERVER_RPC}:7601""##]
#[doc = r##"      - "${APP_SERVER_METRICS}:7602""##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      { interval: 1s, retries: 10, test: "curl -sf http://localhost:${APP_SERVER_ADMIN}/health" }"##]
#[doc = r##""##]
#[doc = r##"init:"##]
#[doc = r##"  deploy-contracts:"##]
#[doc = r##"    # The following line is invalid because the init container `use` key must specify a target"##]
#[doc = r##"    use: ./contracts"##]
#[doc = r##"    depends_on:"##]
#[doc = r##"      chain: { condition: service_healthy }"##]
#[doc = r##"    environment:"##]
#[doc = r##"      CHAIN_ID: ${CHAIN_ID}"##]
#[doc = r##"      CHAIN_RPC: http://chain:${CHAIN_RPC}"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/package_use_no_target_init.yaml`"]
pub const PACKAGE_USE_NO_TARGET_INIT: &str = indoc::indoc! { r###"
  ---
  # Chain config
  CHAIN_RPC = 8545
  CHAIN_ID = 1337
  CHAIN_NAME = "hardhat"

  # App server
  APP_SERVER_ADMIN = 7600
  APP_SERVER_RPC = 7601
  APP_SERVER_METRICS = 7602
  ---

  services:
    chain:
      use: ./chain
      environment:
        CHAIN_RPC: "${CHAIN_RPC}"

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
        { interval: 1s, retries: 10, test: "curl -sf http://localhost:${APP_SERVER_ADMIN}/health" }

  init:
    deploy-contracts:
      # The following line is invalid because the init container `use` key must specify a target
      use: ./contracts
      depends_on:
        chain: { condition: service_healthy }
      environment:
        CHAIN_ID: ${CHAIN_ID}
        CHAIN_RPC: http://chain:${CHAIN_RPC}"### };
