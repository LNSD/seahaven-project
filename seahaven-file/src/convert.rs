//! # Seahaven setup description file transformations
//!
//! This module provides functions for converting Seahaven setup description files
//! into [Compose] and [.env] files.
//!
//! [Compose]: https://github.com/compose-spec/compose-spec/blob/main/spec.md
//! [.env]: https://dotenvx.com/docs/env-file

use crate::{compose::ComposeFile, model::content::Content};

/// Try to convert a [`Content`] into a [`ComposeFile`].
pub fn try_into_compose_file(file: Content) -> Result<ComposeFile, Error> {
    let mut compose_file = ComposeFile {
        name: file.name,
        services: file.services,
        networks: file.networks,
        volumes: file.volumes,
        configs: file.configs,
        secrets: file.secrets,
    };

    // Merge the init-containers into the services
    if let Some(init) = file.init {
        // Check name overlap between init-containers, `init`, and `services`
        let bad_keys = init
            .keys()
            .filter_map(|key| key.as_str())
            .filter(|key| compose_file.services.contains_key(key))
            .collect::<Vec<_>>();
        if !bad_keys.is_empty() {
            return Err(anyhow::anyhow!("The following names overlap: {:?}", bad_keys).into());
        }

        compose_file.services.extend(init);
    }

    Ok(compose_file)
}

/// Error type for conversion operations in the Seahaven file module.
///
/// It's designed to be used as a return type for functions that may fail during
/// the conversion of Seahaven setup description files.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(Box<dyn std::error::Error + Send + Sync>);

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self(err.into())
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::Mapping;

    use super::*;

    const PROJECT_NAME: &str = "test-project";
    const CHAIN_SERVICE_NAME: &str = "chain";
    const DEPLOY_SMART_CONTRACTS_INIT_CONTAINER_NAME: &str = "deploy-smart-contracts";

    fn chain_service() -> (serde_yaml::Value, serde_yaml::Value) {
        let name = CHAIN_SERVICE_NAME.to_string();
        let map = Mapping::from_iter([(
            serde_yaml::Value::String("image".to_string()),
            serde_yaml::Value::String("ghcr.io/foundry-rs/foundry:latest".to_string()),
        )]);
        (
            serde_yaml::Value::String(name),
            serde_yaml::Value::Mapping(map),
        )
    }

    fn deploy_smart_contracts_init_container() -> (serde_yaml::Value, serde_yaml::Value) {
        let name = DEPLOY_SMART_CONTRACTS_INIT_CONTAINER_NAME.to_string();
        let map = Mapping::from_iter([(
            serde_yaml::Value::String("build".to_string()),
            serde_yaml::Value::Mapping(Mapping::from_iter([
                (
                    serde_yaml::Value::String("context".to_string()),
                    serde_yaml::Value::String("contracts".to_string()),
                ),
                (
                    serde_yaml::Value::String("dockerfile".to_string()),
                    serde_yaml::Value::String("Dockerfile.deploy".to_string()),
                ),
            ])),
        )]);
        (
            serde_yaml::Value::String(name),
            serde_yaml::Value::Mapping(map),
        )
    }

    #[test]
    fn init_containers_are_merged_into_services() {
        //* Given
        let services = Mapping::from_iter([chain_service()]);
        let init = Mapping::from_iter([deploy_smart_contracts_init_container()]);

        let content = Content {
            name: Some(PROJECT_NAME.to_string()),
            services,
            init: Some(init),
            networks: None,
            volumes: None,
            configs: None,
            secrets: None,
        };

        //* When
        let result = try_into_compose_file(content);

        //* Then
        assert!(result.is_ok());

        let compose_file = result.expect("Failed to convert content to compose file");

        // Verify the name is preserved
        assert_eq!(compose_file.name, Some(PROJECT_NAME.to_string()));

        // Verify services contains both the chain service and the init container
        assert_eq!(compose_file.services.len(), 2);
        assert!(compose_file.services.contains_key(CHAIN_SERVICE_NAME));
        assert!(
            compose_file
                .services
                .contains_key(DEPLOY_SMART_CONTRACTS_INIT_CONTAINER_NAME)
        );

        // Verify other fields are None
        assert!(compose_file.networks.is_none());
        assert!(compose_file.volumes.is_none());
        assert!(compose_file.configs.is_none());
        assert!(compose_file.secrets.is_none());
    }

    #[test]
    fn overlapping_container_names_error() {
        //* Given
        const OVERLAPPING_NAME: &str = "overlap";

        let services = {
            let (_, service) = chain_service();
            Mapping::from_iter([(
                serde_yaml::Value::String(OVERLAPPING_NAME.to_string()),
                service,
            )])
        };
        let init = {
            let (_, init_container) = deploy_smart_contracts_init_container();
            Mapping::from_iter([(
                serde_yaml::Value::String(OVERLAPPING_NAME.to_string()),
                init_container,
            )])
        };

        let content = Content {
            name: Some(PROJECT_NAME.to_string()),
            services,
            init: Some(init),
            networks: None,
            volumes: None,
            configs: None,
            secrets: None,
        };

        //* When
        let result = try_into_compose_file(content);

        //* Then
        assert!(result.is_err());

        // Verify the error message contains the overlapping name
        let err = result.expect_err("Expected an error");

        let err_msg = err.to_string();
        assert!(err_msg.contains(OVERLAPPING_NAME));
    }
}
