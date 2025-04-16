//! # Command builder
//!
//! This module provides a type-safe abstraction for constructing and executing Docker commands.
//! It allows for programmatic interaction with Docker's CLI without directly relying on string
//! manipulation or process spawning, making Docker operations more maintainable and less error-prone.
//!
//! ## Architecture
//!
//! The module is structured around command builders that implement the `IntoCommand` trait,
//! enabling consistent conversion of builder structs into executable command representations.
//! Each Docker command family is organized into its own submodule.
//!
//! ## Supported Commands
//!
//! The following Docker commands are currently supported:
//!
//! - `docker --version`
//! - `docker version`
//! - `docker system (info | prune)`
//! - `docker compose (pull | build | up | down)`
//!
//! ## Examples
//!
//! ### Basic Docker Command
//!
//! ```rust
//! use seahaven_docker::{
//!     cmd::{DockerCmd, IntoCommand},
//!     exe::resolve_cli_executable,
//! };
//!
//! // Get Docker version information
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let exe = resolve_cli_executable()?;
//! let version_cmd = DockerCmd::with_executable(exe).version();
//! let mut command = version_cmd.into_command();
//!
//! // Execute the command
//! let output = command.output().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Docker Compose Command with Options
//!
//! ```rust
//! use seahaven_docker::{
//!     cmd::{DockerCmd, IntoCommand},
//!     exe::resolve_cli_executable,
//! };
//!
//! // Configure and start services with Docker Compose
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let exe = resolve_cli_executable()?;
//! let mut compose_cmd = DockerCmd::with_executable(exe)
//!     .compose()
//!     .up()
//!     .with_detach(true)
//!     .with_service("my-service");
//! let command = compose_cmd.into_command();
//! # Ok(())
//! # }
//! ```
//!
//! Note that the order of method calls matters - `with_progress_json` must be called before `up`.
//! The reverse order would not compile:
//!
//! ```compilation_error
//! use seahaven_docker::cmd::{DockerCmd, IntoCommand};
//!
//! let mut compose_cmd = DockerCmd::new()
//!     .compose()
//!     .up()
//!     .with_progress_json() // ❌ Error!
//!     .with_detach();
//! ```

mod common;
pub mod compose;
mod root;
mod root_version;
pub mod system;
pub mod version;

pub use common::IntoCommand;
pub use root::DockerCmd;
pub use root_version::DockerRootVersionCmd;

#[cfg(test)]
mod tests {
    mod common;
    mod it_compose;
    mod it_compose_build;
    mod it_compose_down;
    mod it_compose_pull;
    mod it_compose_up;
    mod it_root;
    mod it_system;
    mod it_version;
}
