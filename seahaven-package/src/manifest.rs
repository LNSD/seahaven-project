//! # The Package Manifest
//!
//! The package `manifest.toml` file is used to describe reusable services and init targets
//! in a Seahaven `setup.yaml` file.

mod display;
mod parsing;
mod parts;

pub use display::{SerializationError, to_pretty_string, to_string};
pub use parsing::{DeserializationError, from_str};
pub use parts::*;
