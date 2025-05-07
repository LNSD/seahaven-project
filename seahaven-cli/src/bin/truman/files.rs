//! # Seahaven file processing

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::Arc,
};

use seahaven_cli::{result::Result, serde_yaml_ext::MappingMergeExt, transcoding};
use seahaven_compose_file::ComposeFile;
use seahaven_package::manifest::Manifest;
use seahaven_setup_file::{
    Content, Env,
    model::{PackageUse, Path as PackageUsePath},
};

pub mod env;

/// Loads a single file and returns its environment variables and content.
///
/// Returns an error if the file cannot be opened or parsed.
pub fn load_setup_file(
    path: &impl AsRef<Path>,
    workdir: &impl AsRef<Path>,
) -> Result<(Option<Env>, Content)> {
    let file = File::open(path).map(BufReader::new).map_err(|err| {
        anyhow::anyhow!(
            "Failed to open setup file '{}': {}",
            path.as_ref().display(),
            err
        )
    })?;

    let (env, mut content) = seahaven_setup_file::from_reader(file)
        .map_err(|err| {
            anyhow::anyhow!(
                "Failed to parse setup file '{}': {}",
                path.as_ref().display(),
                err
            )
        })?
        .unpack();

    let mut manifests_cache = HashMap::new();

    // Merge the services' configuration into the service defaults
    for (_, service) in content.services.iter_mut() {
        // Get the package use
        let package_use = match service.package_use.as_ref() {
            Some(package_use) => package_use,
            None => continue,
        };

        let manifest =
            load_package_manifest_with_cache(package_use, workdir, &mut manifests_cache)?;

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
    }

    // Merge the init containers' package default configuration into the init containers' configuration
    if let Some(init) = content.init.as_mut() {
        for (_, init) in init.iter_mut() {
            // Get the package use
            let package_use = match init.package_use.as_ref() {
                Some(package_use) => package_use,
                None => continue,
            };

            let manifest =
                load_package_manifest_with_cache(package_use, workdir, &mut manifests_cache)?;

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
        }
    }

    Ok((env, content))
}

/// Convert a [`Content`] into a [`ComposeFile`].
pub fn into_compose_file(file: Content) -> ComposeFile {
    // Convert the services into a serde_yaml::Mapping
    let mut services = file
        .services
        .into_iter()
        .map(|(name, service)| {
            (
                serde_yaml::Value::String(name.into_inner()),
                serde_yaml::Value::Mapping(service._rest),
            )
        })
        .collect::<serde_yaml::Mapping>();

    // Merge the init-containers into the services map
    if let Some(init) = file.init {
        services.extend(init.into_iter().map(|(name, service)| {
            (
                serde_yaml::Value::String(name.into_inner()),
                serde_yaml::Value::Mapping(service._rest),
            )
        }));
    }

    ComposeFile {
        name: file.name.map(|name| name.into_inner()),
        services,
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
    }
}

/// Loads a package manifest file from the given path relative to the provided base path.
///
/// The base path should be the working directory path.
///
/// If the manifest is already loaded, it is returned from the cache.
fn load_package_manifest_with_cache(
    package_use: &PackageUse,
    workdir: &impl AsRef<Path>,
    cache: &mut HashMap<PackageUsePath, Arc<Manifest>>,
) -> Result<Arc<Manifest>> {
    if let Some(manifest) = cache.get(&package_use.path) {
        return Ok(manifest.clone());
    }

    let manifest = load_manifest_file(&package_use.path, workdir).map(Arc::new)?;

    cache.insert(package_use.path.clone(), manifest.clone());

    Ok(manifest)
}

/// Loads a package manifest file from the given path relative to the provided base path.
///
/// The base path should be the working directory path.
fn load_manifest_file(path: &impl AsRef<Path>, working_dir: &impl AsRef<Path>) -> Result<Manifest> {
    let path = working_dir.as_ref().join(path.as_ref());

    let mut file = File::open(&path).map(BufReader::new).map_err(|err| {
        anyhow::anyhow!("Failed to open manifest file '{}': {}", path.display(), err)
    })?;

    let mut manifest = String::with_capacity(1024);
    file.read_to_string(&mut manifest).map_err(|err| {
        anyhow::anyhow!("Failed to read manifest file '{}': {}", path.display(), err)
    })?;

    let manifest = seahaven_package::manifest::from_str(&manifest).map_err(|err| {
        anyhow::anyhow!(
            "Failed to parse manifest file '{}': {}",
            path.display(),
            err
        )
    })?;

    Ok(manifest)
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
