//! Transcode from one [Serde](https://serde.rs/) format to another.
//!
//! This module provides functionality to "transcode" from an arbitrary Serde
//! `Deserializer` to an arbitrary Serde `Serializer` without needing to
//! collect the entire input into an intermediate form in memory. For example,
//! you could translate a stream of JSON data into a stream of CBOR data, or
//! translate JSON into its pretty-printed form.

// Code borrowed with modifications from: https://github.com/sfackler/serde-transcode/blob/7c8dae5816dae317b0132ee02a35bb5d59b163a7/src/lib.rs
// The original project (sfackler/serde-transcode) is licensed under either of the MIT license or the Apache License 2.0.

mod transcoder;

use serde::ser::Serialize as _;
pub use transcoder::Transcoder;

/// Transcodes from a Serde [`Deserializer`] to a Serde [`Serializer`].
///
/// [`Serializer`]: serde::ser::Serializer
/// [`Deserializer`]: serde::de::Deserializer
pub fn transcode<'de, D, S>(d: D, s: S) -> Result<S::Ok, S::Error>
where
    D: serde::de::Deserializer<'de>,
    S: serde::ser::Serializer,
{
    Transcoder::new(d).serialize(s)
}

#[cfg(test)]
mod tests;
