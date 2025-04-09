//! # Seahaven setup description file model
//!
//! This module provides the core types and functions for working with Seahaven setup description files.

pub mod content;
pub mod env;
mod file;

pub use file::SetupFile;
