use std::{fs::File, io::BufReader, path::Path};

use seahaven_cli::{result::Result, serde_yaml_ext::MappingMergeExt, transcoding};
use seahaven_package::loader::{FileLoader as PackageFileLoader, Loader as PackageLoader};
use seahaven_setup_file::{
    Content as SetupSpec, Env,
    model::{InitContainer, Service},
};

/// Loads a single file and returns its environment variables and content.
///
/// Returns an error if the file cannot be opened or parsed.
pub fn load_setup_file(
    path: &impl AsRef<Path>,
    workdir: &impl AsRef<Path>,
) -> Result<(Option<Env>, SetupSpec)> {
    let file = File::open(path).map(BufReader::new).map_err(|err| {
        anyhow::anyhow!(
            "Failed to open setup file '{}': {}",
            path.as_ref().display(),
            err
        )
    })?;

    let (env, mut setup_spec) = seahaven_setup_file::from_reader(file)
        .map_err(|err| {
            anyhow::anyhow!(
                "Failed to parse setup file '{}': {}",
                path.as_ref().display(),
                err
            )
        })?
        .unpack();

    let package_loader = PackageFileLoader::new(workdir.as_ref().to_path_buf());

    // Merge the services' configuration into the service defaults
    for (_, service) in setup_spec.services.iter_mut() {
        load_and_merge_service_config(service, &package_loader)?;
    }

    // Merge the init containers' package default configuration into the init containers' configuration
    if let Some(init) = setup_spec.init.as_mut() {
        for (_, init) in init.iter_mut() {
            load_and_merge_init_container_config(init, &package_loader)?;
        }
    }

    Ok((env, setup_spec))
}

/// Loads and merges package configuration for a service
fn load_and_merge_service_config(
    service: &mut Service,
    package_loader: &impl PackageLoader,
) -> Result<()> {
    // If the service has no package use, we're done
    let package_use = match service.package_use.as_ref() {
        Some(package_use) => package_use,
        None => return Ok(()),
    };

    let manifest = package_loader.load(&package_use.path).map_err(|err| {
        anyhow::anyhow!(
            "Failed to load manifest file '{}': {}",
            package_use.path.display(),
            err
        )
    })?;

    // Check the package use target:
    // - The target must be specified in the manifest (as a service)
    // - If the target is not specified, it means the target is the service. If the service
    //   is not found in the manifest, an error is returned.
    let manifest_target = match &package_use.target {
        Some(target) => {
            if let Some(service) = manifest
                .service
                .as_ref()
                .filter(|service| service.name == target)
            {
                service
            } else {
                return Err(anyhow::anyhow!(
                    "Service target '{}' not found in manifest file '{}'",
                    target,
                    package_use.path.display()
                )
                .into());
            }
        }
        None => {
            if let Some(service) = &manifest.service {
                service
            } else {
                return Err(anyhow::anyhow!(
                    "Service target not specified in manifest file '{}'",
                    package_use.path.display()
                )
                .into());
            }
        }
    };

    let mut service_config = {
        let mut mapping = serde_yaml::Mapping::new();
        for (key, value) in manifest_target.defaults.iter() {
            match transcode_package_target_defaults(value.clone()) {
                Ok(transcoded) => {
                    mapping.insert(serde_yaml::Value::String(key.to_owned()), transcoded);
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "Failed to transcode package target defaults: {}",
                        err
                    )
                    .into());
                }
            }
        }

        mapping
    };

    // Merge the service's configuration into the service defaults
    service_config.merge(&service._rest);

    // Update the service's configuration
    service._rest = service_config;

    Ok(())
}

/// Loads and merges package configuration for an init container
fn load_and_merge_init_container_config(
    init: &mut InitContainer,
    package_loader: &impl PackageLoader,
) -> Result<()> {
    // If the init container has no package use, we're done
    let package_use = match init.package_use.as_ref() {
        Some(package_use) => package_use,
        None => return Ok(()),
    };

    let manifest = package_loader.load(&package_use.path).map_err(|err| {
        anyhow::anyhow!(
            "Failed to load manifest file '{}': {}",
            package_use.path.display(),
            err
        )
    })?;

    // Check the package use target:
    // - The target must be specified in the manifest (init container)
    // - If the target is not specified, an error is returned.
    let manifest_target = match &package_use.target {
        Some(target) => {
            if let Some(init) = manifest.init.iter().find(|init| init.name == target) {
                init
            } else {
                return Err(anyhow::anyhow!(
                    "Init container target '{}' not found in manifest file '{}'",
                    target,
                    package_use.path.display()
                )
                .into());
            }
        }
        None => {
            return Err(anyhow::anyhow!(
                "Init container target required for 'use' key '{}'",
                package_use.path.display()
            )
            .into());
        }
    };

    let mut init_config = {
        let mut mapping = serde_yaml::Mapping::new();
        for (key, value) in manifest_target.defaults.iter() {
            match transcode_package_target_defaults(value.clone()) {
                Ok(transcoded) => {
                    mapping.insert(serde_yaml::Value::String(key.to_owned()), transcoded);
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "Failed to transcode package target defaults: {}",
                        err
                    )
                    .into());
                }
            }
        }

        mapping
    };

    // Merge the init's configuration into the init defaults
    init_config.merge(&init._rest);

    // Update the init's configuration
    init._rest = init_config;

    Ok(())
}

/// This function converts the `[service.defaults]` or `[[init.defaults]]` table into
/// a setup-file's `service` or `init` fields.
///
/// Transcodes a [`toml::Value`] to a [`serde_yaml::Value`].
fn transcode_package_target_defaults(
    value: toml::Value,
) -> Result<serde_yaml::Value, serde_yaml::Error> {
    transcoding::transcode(value, serde_yaml::value::Serializer)
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::Arc};

    use seahaven_package::{
        loader::{Error, Loader as PackageLoader},
        manifest::{self, Manifest},
    };

    use super::{load_and_merge_init_container_config, load_and_merge_service_config};

    /// Creates a test manifest with the given package name and init container
    fn test_manifest(
        package_name: &str,
        service: impl Into<Option<manifest::Service>>,
        init: impl IntoIterator<Item = manifest::InitContainer>,
    ) -> Manifest {
        Manifest {
            package: manifest::PackageMeta {
                name: package_name.parse().expect("package name is not valid"),
                version: None,
                description: None,
                readme: None,
            },
            service: service.into(),
            init: init.into_iter().collect(),
        }
    }

    /// Creates a test manifest service with the given name and defaults
    fn test_manifest_service(
        name: &str,
        defaults: Vec<(String, toml::Value)>,
    ) -> manifest::Service {
        manifest::Service {
            name: name.parse().expect("service name is not valid"),
            defaults: defaults.into_iter().collect(),
            _rest: toml::Table::new(),
        }
    }

    /// Creates a test manifest init container with the given name and defaults
    fn test_manifest_init_container(
        name: &str,
        defaults: Vec<(String, toml::Value)>,
    ) -> manifest::InitContainer {
        manifest::InitContainer {
            name: name.parse().expect("init container name is not valid"),
            defaults: defaults.into_iter().collect(),
            _rest: toml::Table::new(),
        }
    }

    /// Create a test setup file service with the given package use
    fn test_setup_file_service(
        package_use: impl Into<Option<&'static str>>,
        rest: serde_yaml::Mapping,
    ) -> seahaven_setup_file::model::Service {
        seahaven_setup_file::model::Service {
            package_use: package_use
                .into()
                .map(|s| s.parse().expect("Invalid package use value")),
            _rest: rest,
        }
    }

    /// Create a test setup file init container with the given package use
    fn test_setup_file_init_container(
        package_use: impl Into<Option<&'static str>>,
        rest: serde_yaml::Mapping,
    ) -> seahaven_setup_file::model::InitContainer {
        seahaven_setup_file::model::InitContainer {
            package_use: package_use
                .into()
                .map(|s| s.parse().expect("Invalid package use value")),
            _rest: rest,
        }
    }

    mockall::mock! {
        Loader {}

        impl PackageLoader for Loader {
            #[mockall::concretize]
            fn load<P: AsRef<Path>>(&self, path: P) -> Result<Arc<Manifest>, Error>;
        }
    }

    mod service {
        use super::*;

        #[test]
        fn empty_config_when_no_package_use() {
            //* Given
            let mut service = test_setup_file_service(None, Default::default());
            let mock_loader = MockLoader::new();

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert!(service._rest.is_empty(), "service config should be empty");
        }

        #[test]
        fn merges_defaults_with_valid_target() {
            //* Given
            let mut service =
                test_setup_file_service("test-package#target-service", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    test_manifest_service(
                        "target-service",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    ),
                    [],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert_eq!(service._rest.get("key").unwrap().as_str().unwrap(), "value");
        }

        #[test]
        fn errors_when_target_not_found() {
            //* Given
            let mut service =
                test_setup_file_service("test-package#non-existent", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    test_manifest_service(
                        "target-service",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    ),
                    [],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("not found in manifest file")
            );
        }

        #[test]
        fn uses_default_service_when_no_target() {
            //* Given
            let mut service = test_setup_file_service("test-package", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    test_manifest_service(
                        "target-service",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    ),
                    [],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert_eq!(service._rest.get("key").unwrap().as_str().unwrap(), "value");
        }

        #[test]
        fn preserves_existing_config_when_merging() {
            //* Given
            let mut service = test_setup_file_service(
                "test-package#target-service",
                serde_yaml::Mapping::from_iter([(
                    serde_yaml::Value::String("existing".to_string()),
                    serde_yaml::Value::String("value".to_string()),
                )]),
            );

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    test_manifest_service(
                        "target-service",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    ),
                    [],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert_eq!(service._rest.get("key").unwrap().as_str().unwrap(), "value");
            assert_eq!(
                service._rest.get("existing").unwrap().as_str().unwrap(),
                "value"
            );
        }

        #[test]
        fn errors_when_manifest_load_fails() {
            //* Given
            let mut service =
                test_setup_file_service("test-package#target-service", Default::default());

            let mock_loader = {
                let mut loader = MockLoader::new();
                loader.expect_load().returning(|path: &dyn AsRef<Path>| {
                    Err(Error::FileOpen {
                        path: path.as_ref().to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
                    })
                });
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Failed to load manifest file"),
                "result should contain 'Failed to load manifest file'"
            );
        }

        #[test]
        fn errors_when_no_service_in_manifest() {
            //* Given
            let mut service = test_setup_file_service("test-package", Default::default());

            let mock_loader = {
                let manifest = test_manifest("test-package", None, []);

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_service_config(&mut service, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Service target not specified in manifest file"),
                "result should contain 'Service target not specified in manifest file'"
            );
        }
    }

    mod init_container {
        use super::*;

        #[test]
        fn empty_config_when_no_package_use() {
            //* Given
            let mut init = test_setup_file_init_container(None, Default::default());
            let mock_loader = MockLoader::new();

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert!(init._rest.is_empty(), "init config should be empty");
        }

        #[test]
        fn merges_defaults_with_valid_target() {
            //* Given
            let mut init =
                test_setup_file_init_container("test-package#target-init", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    None,
                    [test_manifest_init_container(
                        "target-init",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    )],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(
                result.is_ok(),
                "load and merge config should not return an error"
            );
            assert_eq!(init._rest.get("key").unwrap().as_str().unwrap(), "value");
        }

        #[test]
        fn errors_when_target_not_found() {
            //* Given
            let mut init =
                test_setup_file_init_container("test-package#non-existent", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    None,
                    [test_manifest_init_container(
                        "target-init",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    )],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("not found in manifest file"),
                "result should contain 'not found in manifest file'"
            );
        }

        #[test]
        fn errors_when_no_target_specified() {
            //* Given
            let mut init = test_setup_file_init_container("test-package", Default::default());

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    None,
                    [test_manifest_init_container(
                        "target-init",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    )],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Init container target required"),
                "result should contain 'Init container target required'"
            );
        }

        #[test]
        fn preserves_existing_config_when_merging() {
            //* Given
            let mut init = test_setup_file_init_container(
                "test-package#target-init",
                serde_yaml::Mapping::from_iter([(
                    serde_yaml::Value::String("existing".to_string()),
                    serde_yaml::Value::String("value".to_string()),
                )]),
            );

            let mock_loader = {
                let manifest = test_manifest(
                    "test-package",
                    None,
                    [test_manifest_init_container(
                        "target-init",
                        vec![("key".to_string(), toml::Value::String("value".to_string()))],
                    )],
                );

                let mut loader = MockLoader::new();
                loader
                    .expect_load()
                    .return_once(move |_: &dyn AsRef<Path>| Ok(Arc::new(manifest)));
                loader
            };

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(result.is_ok(), "load and merge config should succeed");
            assert_eq!(init._rest.get("key").unwrap().as_str().unwrap(), "value");
            assert_eq!(
                init._rest.get("existing").unwrap().as_str().unwrap(),
                "value"
            );
        }

        #[test]
        fn errors_when_manifest_load_fails() {
            //* Given
            let mut init =
                test_setup_file_init_container("test-package#target-init", Default::default());

            let mock_loader = {
                let mut loader = MockLoader::new();
                loader.expect_load().returning(|path: &dyn AsRef<Path>| {
                    Err(Error::FileOpen {
                        path: path.as_ref().to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
                    })
                });
                loader
            };

            //* When
            let result = load_and_merge_init_container_config(&mut init, &mock_loader);

            //* Then
            assert!(
                result.is_err(),
                "load and merge config should return an error"
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Failed to load manifest file"),
                "result should contain 'Failed to load manifest file'"
            );
        }
    }
}
