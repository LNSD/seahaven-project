//! # Command builder
//!
//! This module provides a type-safe abstraction for constructing and executing Just commands.
//! It allows for programmatic interaction with Just's CLI without directly relying on string
//! manipulation or process spawning, making Just operations more maintainable and less error-prone.
//!
//! ## Architecture
//!
//! The module is structured around command builders that implement the `IntoCommand` trait,
//! enabling consistent conversion of builder structs into executable command representations.
//! Each Just command family is organized into its own submodule.
//!
//! ## Supported Commands
//!
//! The following Just commands are currently supported:
//!
//! - `just --version`
//! - `just --dump`
//! - `just --list`
//! - `just --summary`
//! - `just` with the following options:
//!   - `--justfile <path>` - Specify an alternate justfile
//!   - `--dotenv-path <path>` - Specify an alternate environment file
//!   - `--working-directory <path>` - Specify a working directory
//!   - `--dry-run` - Enable dry-run mode
//!   - `[ARGUMENTS]...` - Overrides and recipe(s) to run

mod common;
pub mod dump;
pub mod list;
mod root;
pub mod summary;
pub mod version;

pub use common::IntoCommand;
pub use root::{
    EnvFileNotSet, EnvFileOpt, EnvFileSet, JustCmd, JustfileNotSet, JustfileOpt, JustfileSet,
};

#[cfg(test)]
mod tests {
    mod common;
    mod it_dump;
    mod it_list;
    mod it_root;
    mod it_summary;
    mod it_version;
}
