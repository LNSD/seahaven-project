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

pub mod compose;
pub mod content;
pub mod env;
mod matter;

use std::io::{BufRead, Seek};

use self::{content::FileContent, env::EnvFile};

/// Parses a Seahaven setup description YAML file from a IO stream
///
/// This function extracts and parses both the front-matter section (if present) and the main YAML content.
/// The front-matter is parsed into an optional [`EnvFile`] containing environment variables,
/// while the remaining content is parsed into a [`FileContent`].
///
/// See [`Error`] for the errors that can occur when parsing a setup description file
pub fn from_reader<R>(reader: R) -> Result<(Option<EnvFile>, FileContent), ParsingError>
where
    R: BufRead + Seek,
{
    let (front_matter, content) = matter::extract_front_matter(reader)?;

    let env_file = front_matter.map(serde_envfile::from_reader).transpose()?;
    let file_content = content::de::from_reader(content)?;

    Ok((env_file, file_content))
}

/// Errors that can occur when parsing a setup description file
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ParsingError {
    /// The front matter section of a file has an invalid format
    #[error("invalid front-matter format: {0}")]
    InvalidFrontMatterFormat(#[from] matter::Error),

    /// The environment variables in the front-matter section cannot be parsed
    #[error("environment variables parsing failed: {0}")]
    EnvParsingFailed(#[from] serde_envfile::Error),

    /// The main content of the setup description file cannot be parsed
    #[error("content deserialization failed: {0}")]
    ContentDeserializationFailed(#[from] content::de::DeserializationError),
}
