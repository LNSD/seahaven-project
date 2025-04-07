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
/// CHAIN_RPC=8545
/// CHAIN_ID=1337
/// CHAIN_NAME="hardhat"
///
/// # App server
/// APP_SERVER_ADMIN=7600
/// APP_SERVER_RPC=7601
/// APP_SERVER_METRICS=7602
///
/// # Database config
/// DB_PASSWORD=secret
/// APP_PORT=8080
/// ---
/// name: kitchen-sink-project
///
/// services:
///   chain:
///     image: ghcr.io/foundry-rs/foundry:latest
///     command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
///     ports:
///       - "${CHAIN_RPC}:8545"
///     networks:
///       - blockchain
///     healthcheck:
///       { interval: 1s, retries: 10, test: cast block }
///
///   app-server:
///     image: ghcr.io/example/server:latest
///     depends_on:
///       chain: { condition: service_healthy }
///       deploy-contracts: { condition: service_completed_successfully }
///       db: { condition: service_healthy }
///     ports:
///       - "${APP_SERVER_ADMIN}:7600"
///       - "${APP_SERVER_RPC}:7601"
///       - "${APP_SERVER_METRICS}:7602"
///     networks:
///       - frontend
///       - backend
///       - blockchain
///     volumes:
///       - app-data:/data
///     configs:
///       - source: app-config
///         target: /app/config.yaml
///     secrets:
///       - source: db-password
///         target: /app/db/password
///     healthcheck:
///       { interval: 1s, retries: 10, test: "curl -sf http://localhost:${APP_SERVER_ADMIN}/health" }
///
///   db:
///     image: postgres:14
///     environment:
///       POSTGRES_PASSWORD: "${DB_PASSWORD}"
///     networks:
///       - backend
///     volumes:
///       - db-data:/var/lib/postgresql/data
///     healthcheck:
///       test: ["CMD-SHELL", "pg_isready -U postgres"]
///       interval: 10s
///       timeout: 5s
///       retries: 5
///
/// init-containers:
///   deploy-contracts:
///     build: { context: contracts }
///     depends_on:
///       chain: { condition: service_healthy }
///     volumes:
///       - ./contracts.json:/opt/contracts.json:ro
///
/// networks:
///   frontend:
///     driver: bridge
///   backend:
///     driver: bridge
///   blockchain:
///     driver: bridge
///
/// volumes:
///   app-data:
///   db-data:
///
/// configs:
///   app-config:
///     file: ./config/app.yaml
///
/// secrets:
///   db-password:
///     file: ./secrets/db-password.txt
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
  CHAIN_RPC=8545
  CHAIN_ID=1337
  CHAIN_NAME="hardhat"

  # App server
  APP_SERVER_ADMIN=7600
  APP_SERVER_RPC=7601
  APP_SERVER_METRICS=7602

  # Database config
  DB_PASSWORD=secret
  APP_PORT=8080
  ---
  name: kitchen-sink-project

  services:
    chain:
      image: ghcr.io/foundry-rs/foundry:latest
      command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
      ports:
        - "${CHAIN_RPC}:8545"
      networks:
        - blockchain
      healthcheck:
        { interval: 1s, retries: 10, test: cast block }

    app-server:
      image: ghcr.io/example/server:latest
      depends_on:
        chain: { condition: service_healthy }
        deploy-contracts: { condition: service_completed_successfully }
        db: { condition: service_healthy }
      ports:
        - "${APP_SERVER_ADMIN}:7600"
        - "${APP_SERVER_RPC}:7601"
        - "${APP_SERVER_METRICS}:7602"
      networks:
        - frontend
        - backend
        - blockchain
      volumes:
        - app-data:/data
      configs:
        - source: app-config
          target: /app/config.yaml
      secrets:
        - source: db-password
          target: /app/db/password
      healthcheck:
        { interval: 1s, retries: 10, test: "curl -sf http://localhost:${APP_SERVER_ADMIN}/health" }

    db:
      image: postgres:14
      environment:
        POSTGRES_PASSWORD: "${DB_PASSWORD}"
      networks:
        - backend
      volumes:
        - db-data:/var/lib/postgresql/data
      healthcheck:
        test: ["CMD-SHELL", "pg_isready -U postgres"]
        interval: 10s
        timeout: 5s
        retries: 5

  init-containers:
    deploy-contracts:
      build: { context: contracts }
      depends_on:
        chain: { condition: service_healthy }
      volumes:
        - ./contracts.json:/opt/contracts.json:ro

  networks:
    frontend:
      driver: bridge
    backend:
      driver: bridge
    blockchain:
      driver: bridge

  volumes:
    app-data:
    db-data:

  configs:
    app-config:
      file: ./config/app.yaml

  secrets:
    db-password:
      file: ./secrets/db-password.txt"### };
