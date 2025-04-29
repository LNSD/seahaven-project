//! # Seahaven setup description file conversion
//!
//! This module provides functions for converting Seahaven setup description files
//! into [Compose] files.
//!
//! [Compose]: https://github.com/compose-spec/compose-spec/blob/main/spec.md

use std::convert::Infallible;

use seahaven_compose_file::ComposeFile;

use crate::model::Content;

/// Convert a [`Content`] into a [`ComposeFile`].
pub fn try_into_compose_file(file: Content) -> Result<ComposeFile, Infallible> {
    let mut compose_file = ComposeFile {
        name: file.name.map(|name| name.into_inner()),
        services: file
            .services
            .into_iter()
            .map(|(name, service)| (serde_yaml::Value::String(name.into_inner()), service))
            .collect(),
        networks: file
            ._rest
            .get("networks")
            .and_then(|networks| networks.as_mapping())
            .cloned(),
        volumes: file
            ._rest
            .get("volumes")
            .and_then(|volumes| volumes.as_mapping())
            .cloned(),
        configs: file
            ._rest
            .get("configs")
            .and_then(|configs| configs.as_mapping())
            .cloned(),
        secrets: file
            ._rest
            .get("secrets")
            .and_then(|secrets| secrets.as_mapping())
            .cloned(),
    };

    // Merge the init-containers into the services map
    if let Some(init) = file.init {
        compose_file.services.extend(
            init.into_iter()
                .map(|(name, service)| (serde_yaml::Value::String(name.into_inner()), service)),
        );
    }

    Ok(compose_file)
}
