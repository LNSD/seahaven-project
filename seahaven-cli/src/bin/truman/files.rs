//! # Seahaven file processing

use std::{fs::File, io::BufReader, path::Path};

use seahaven_cli::{result::Result, transcoding};
use seahaven_compose_file::ComposeFile;
use seahaven_setup_file::{Content, Env};

/// Loads a single file and returns its environment variables and content.
///
/// Returns an error if the file cannot be opened or parsed.
pub fn load_setup_file<P>(path: P) -> Result<(Option<Env>, Content)>
where
    P: AsRef<Path>,
{
    let file = File::open(&path).map(BufReader::new).map_err(|err| {
        anyhow::anyhow!(
            "Failed to open setup file '{}': {}",
            path.as_ref().display(),
            err
        )
    })?;

    let res = seahaven_setup_file::from_reader(file)
        .map_err(|err| {
            anyhow::anyhow!(
                "Failed to parse setup file '{}': {}",
                path.as_ref().display(),
                err
            )
        })?
        .unpack();

    Ok(res)
}

/// Loads all variables found in the files into the environment,
/// overriding any existing environment variables of the same name.
///
/// If a variable is specified multiple times in different files,
/// then the last occurrence is applied.
///
/// Files are loaded in order, with later values overriding earlier ones.
/// If a file is invalid or unreadable, an error is returned.
pub fn load_env_files<P>(files: impl IntoIterator<Item = P>) -> Result<Option<Env>>
where
    P: AsRef<Path>,
{
    let mut env = Env::new();

    // Load all files into the environment
    for path in files {
        let reader = File::open(&path).map(BufReader::new).map_err(|err| {
            anyhow::anyhow!(
                "Failed to open env file '{}': {}",
                path.as_ref().display(),
                err,
            )
        })?;

        for pair in dotenvy::Iter::new(reader) {
            match pair {
                Ok((key, value)) => {
                    env.insert(key, value);
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "Failed to parse env file '{}': {}",
                        path.as_ref().display(),
                        err
                    )
                    .into());
                }
            }
        }
    }

    if env.is_empty() {
        return Ok(None);
    }

    Ok(Some(env))
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
                serde_yaml::Value::Mapping(
                    service
                        ._rest
                        .into_iter()
                        .map(|(k, v)| (serde_yaml::Value::String(k), v))
                        .collect(),
                ),
            )
        })
        .collect::<serde_yaml::Mapping>();

    // Merge the init-containers into the services map
    if let Some(init) = file.init {
        services.extend(init.into_iter().map(|(name, service)| {
            (
                serde_yaml::Value::String(name.into_inner()),
                serde_yaml::Value::Mapping(
                    service
                        ._rest
                        .into_iter()
                        .map(|(k, v)| (serde_yaml::Value::String(k), v))
                        .collect(),
                ),
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

/// This function converts the `[service.defaults]` or `[[init.defaults]]` table into
/// a setup-file's `service` or `init` fields.
///
/// Transcodes a [`toml::Value`] to a [`serde_yaml::Value`].
pub fn transcode_package_target_defaults(
    value: toml::Value,
) -> Result<serde_yaml::Value, serde_yaml::Error> {
    transcoding::transcode(value, serde_yaml::value::Serializer)
}
