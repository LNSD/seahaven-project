//! # Seahaven file processing

use std::path::{Path, PathBuf};

use seahaven_cli::result::Result;
use seahaven_compose_file::ComposeFile;
use seahaven_setup_file::Content;

pub mod env;
pub mod setup_yaml;

/// Resolves the setup file and project directory paths.
///
/// The setup file path is canonicalized and verified to exist. If the project directory
/// is not provided, it defaults to the parent directory of the setup file, or the current
/// directory if the setup file has no parent.
///
/// Returns a tuple containing the canonicalized setup file path and the resolved project
/// directory path. Returns an error if the setup file does not exist or the absolute path
/// cannot be resolved.
pub fn resolve_setup_file_and_project_dir_paths(
    setup_file: impl AsRef<Path>,
    project_directory: Option<impl AsRef<Path>>,
) -> Result<(PathBuf, PathBuf)> {
    let setup_file = setup_file.as_ref();
    let setup_file = match setup_file.canonicalize() {
        Ok(path) => path,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                return Err(
                    anyhow::anyhow!("Setup file not found: {}", setup_file.display()).into(),
                );
            }
            _ => {
                return Err(anyhow::anyhow!("Failed to resolve setup file path: {err}").into());
            }
        },
    };

    tracing::debug!("Setup file: {}", setup_file.display());

    // Get the project directory, if not provided, use the parent directory of the setup file,
    // otherwise use the current directory
    let project_directory = match project_directory {
        Some(project_directory) => project_directory.as_ref().to_path_buf(),
        None => setup_file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from(".")),
    };

    tracing::debug!("Project directory: {}", project_directory.display());

    Ok((setup_file, project_directory))
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
