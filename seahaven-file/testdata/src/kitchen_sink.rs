#[doc = "Test vector: `seahaven-file/testdata/data/kitchen_sink`"]
#[doc = ""]
#[doc = "```yaml"]
#[doc = r##"# The following YAML file is a "kitchen sink" of sorts, containing a"##]
#[doc = r##"# variety of features and edge cases that are commonly encountered in"##]
#[doc = r##"# the Seahaven setup description file."##]
#[doc = r##"#"##]
#[doc = r##"# What is exact meaning of "kitchen sink" in programming?"##]
#[doc = r##"# https://stackoverflow.com/q/33779296/1099999"##]
#[doc = r##""##]
#[doc = r##"---"##]
#[doc = r##"# Chain config"##]
#[doc = r##"CHAIN_RPC=8545"##]
#[doc = r##"CHAIN_ID=1337"##]
#[doc = r##"CHAIN_NAME="hardhat""##]
#[doc = r##""##]
#[doc = r##"# App server"##]
#[doc = r##"APP_SERVER_ADMIN=7600"##]
#[doc = r##"APP_SERVER_RPC=7601"##]
#[doc = r##"APP_SERVER_METRICS=7602"##]
#[doc = r##""##]
#[doc = r##"# Database config"##]
#[doc = r##"DB_PASSWORD=secret"##]
#[doc = r##"APP_PORT=8080"##]
#[doc = r##"---"##]
#[doc = r##"name: kitchen-sink-project"##]
#[doc = r##""##]
#[doc = r##"services:"##]
#[doc = r##"  chain:"##]
#[doc = r##"    image: ghcr.io/foundry-rs/foundry:latest"##]
#[doc = r##"    command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0""##]
#[doc = r##"    ports:"##]
#[doc = r##"      - "${CHAIN_RPC}:8545""##]
#[doc = r##"    networks:"##]
#[doc = r##"      - blockchain"##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      { interval: 1s, retries: 10, test: cast block }"##]
#[doc = r##""##]
#[doc = r##"  app-server:"##]
#[doc = r##"    image: ghcr.io/example/server:latest"##]
#[doc = r##"    depends_on:"##]
#[doc = r##"      chain: { condition: service_healthy }"##]
#[doc = r##"      deploy-contracts: { condition: service_completed_successfully }"##]
#[doc = r##"      db: { condition: service_healthy }"##]
#[doc = r##"    ports:"##]
#[doc = r##"      - "${APP_SERVER_ADMIN}:7600""##]
#[doc = r##"      - "${APP_SERVER_RPC}:7601""##]
#[doc = r##"      - "${APP_SERVER_METRICS}:7602""##]
#[doc = r##"    networks:"##]
#[doc = r##"      - frontend"##]
#[doc = r##"      - backend"##]
#[doc = r##"      - blockchain"##]
#[doc = r##"    volumes:"##]
#[doc = r##"      - app-data:/data"##]
#[doc = r##"    configs:"##]
#[doc = r##"      - source: app-config"##]
#[doc = r##"        target: /app/config.yaml"##]
#[doc = r##"    secrets:"##]
#[doc = r##"      - source: db-password"##]
#[doc = r##"        target: /app/db/password"##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      { interval: 1s, retries: 10, test: "curl -sf http://localhost:${APP_SERVER_ADMIN}/health" }"##]
#[doc = r##""##]
#[doc = r##"  db:"##]
#[doc = r##"    image: postgres:14"##]
#[doc = r##"    environment:"##]
#[doc = r##"      POSTGRES_PASSWORD: "${DB_PASSWORD}""##]
#[doc = r##"    networks:"##]
#[doc = r##"      - backend"##]
#[doc = r##"    volumes:"##]
#[doc = r##"      - db-data:/var/lib/postgresql/data"##]
#[doc = r##"    healthcheck:"##]
#[doc = r##"      test: ["CMD-SHELL", "pg_isready -U postgres"]"##]
#[doc = r##"      interval: 10s"##]
#[doc = r##"      timeout: 5s"##]
#[doc = r##"      retries: 5"##]
#[doc = r##""##]
#[doc = r##"init-containers:"##]
#[doc = r##"  deploy-contracts:"##]
#[doc = r##"    build: { context: contracts }"##]
#[doc = r##"    depends_on:"##]
#[doc = r##"      chain: { condition: service_healthy }"##]
#[doc = r##"    volumes:"##]
#[doc = r##"      - ./contracts.json:/opt/contracts.json:ro"##]
#[doc = r##""##]
#[doc = r##"networks:"##]
#[doc = r##"  frontend:"##]
#[doc = r##"    driver: bridge"##]
#[doc = r##"  backend:"##]
#[doc = r##"    driver: bridge"##]
#[doc = r##"  blockchain:"##]
#[doc = r##"    driver: bridge"##]
#[doc = r##""##]
#[doc = r##"volumes:"##]
#[doc = r##"  app-data:"##]
#[doc = r##"  db-data:"##]
#[doc = r##""##]
#[doc = r##"configs:"##]
#[doc = r##"  app-config:"##]
#[doc = r##"    file: ./config/app.yaml"##]
#[doc = r##""##]
#[doc = r##"secrets:"##]
#[doc = r##"  db-password:"##]
#[doc = r##"    file: ./secrets/db-password.txt"##]
#[doc = "```"]
#[doc = ""]
#[doc = "See file: `seahaven-file/testdata/data/kitchen_sink.yaml`"]
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
