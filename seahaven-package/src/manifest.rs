//! # The Package Manifest
//!
//! The package `manifest.toml` file is used to describe reusable services and init targets
//! in a Seahaven `setup.yaml` file.

mod model;

#[cfg(feature = "display")]
pub mod ser;

#[cfg(feature = "parse")]
pub mod de;

pub use model::*;
