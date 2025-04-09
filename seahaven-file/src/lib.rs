//! # Seahaven setup description file
//!
//! The setup description file is a YAML file that describes the setup of a Seahaven
//! environment. It is used to generate the `docker-compose.yml` file and the `.env` file.
//!
//! ## File Format
//!
//! The file consists of two parts:
//! - An optional front-matter section containing environment variables (enclosed between `---` lines)
//! - A main YAML section based on the Compose file format
//!
//! The front-matter section defines environment variables that will be placed in the `.env` file.
//! The main YAML section will be transpiled into a valid compose file.
//!
//! ## Example
//!
//! ```yaml
//! ---
//! # Chain config
//! CHAIN_RPC: 8545
//! CHAIN_ID: 1337
//! CHAIN_NAME: "hardhat"
//!
//! # App server
//! APP_SERVER_ADMIN: 7600
//! APP_SERVER_RPC: 7601
//! APP_SERVER_METRICS: 7602
//! ---
//!
//! services:
//!   chain:
//!     image: ghcr.io/foundry-rs/foundry:latest
//!     command: "anvil --host=0.0.0.0 --chain-id=${CHAIN_ID} --base-fee=0"
//!     ports:
//!       - "${CHAIN_RPC}:8545"
//!     healthcheck:
//!       { interval: 1s, retries: 10, test: cast block }
//!
//!   app-server:
//!     image: ghcr.io/example/server:latest
//!     depends_on:
//!       chain: { condition: service_healthy }
//!       deploy-contracts: { condition: service_completed_successfully }
//!     ports:
//!       - "${APP_SERVER_ADMIN}:7600"
//!       - "${APP_SERVER_RPC}:7601"
//!       - "${APP_SERVER_METRICS}:7602"
//!     healthcheck:
//!       { interval: 1s, retries: 10, test: curl -f http://localhost:${APP_SERVER_ADMIN}/health }
//!
//! init-containers:
//!   deploy-contracts:
//!     build: { context: contracts }
//!     depends_on:
//!       chain: { condition: service_healthy }
//!     volumes:
//!       - ./contracts.json:/opt/contracts.json:ro
//! ```
//!
//! Based on the [Compose file format][compose-spec], the setup file allows
//! you to define your seahaven workspace by declaring the different components
//! and their dependencies.
//!
//! [compose-spec]: https://github.com/compose-spec/compose-spec/blob/main/spec.md

// Re-export serde_envfile
pub use serde_envfile;

pub mod compose; // TODO: Move to seahaven-docker or seahaven-compose-spec
mod matter;
pub mod model;
mod parsing;

pub use model::{SetupFile, content::Content, env::Env};
pub use parsing::{ParsingError, fileenv_from_reader, from_reader};

#[cfg(test)]
mod tests {
    mod it_parsing;
}
