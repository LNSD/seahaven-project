use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use parking_lot::RwLock;

use crate::manifest::{DeserializationError, Manifest};

/// A trait for loading manifests from various sources.
///
/// Implementations should handle caching and error handling for manifest loading operations.
pub trait Loader {
    /// Loads a manifest from the specified path.
    ///
    /// The path format is implementation-specific.
    ///
    /// Returns an Arc-wrapped [`Manifest`] on success, or an error if loading fails.
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Arc<Manifest>, Error>;
}

/// Errors that can occur when loading manifests
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to open a manifest file
    #[error("Failed to open manifest file '{path}': {source}")]
    FileOpen {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to read a manifest file
    #[error("Failed to read manifest file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse a manifest file
    #[error("Failed to parse manifest file '{path}': {source}")]
    ManifestParse {
        path: PathBuf,
        #[source]
        source: DeserializationError,
    },
}

/// A file-based implementation of the [`Loader`] trait that loads manifests from the filesystem.
/// Maintains an in-memory cache of loaded manifests to improve performance.
#[derive(Debug, Clone)]
pub struct FileLoader<M> {
    manifest_loader: M,
    cache: Arc<RwLock<HashMap<PathBuf, Arc<Manifest>>>>,
}

impl FileLoader<ManifestFileLoader> {
    /// Creates a new [`FileLoader`] with the specified root directory.
    ///
    /// All manifest paths will be resolved relative to this root directory.
    pub fn new(root: impl Into<Box<Path>>) -> Self {
        Self {
            manifest_loader: ManifestFileLoader::new(root),
            cache: Default::default(),
        }
    }
}

impl<M> FileLoader<M>
where
    M: ManifestLoader,
{
    /// Creates a new [`FileLoader`] with the specified manifest loader.
    ///
    /// This is a test-helper function.
    #[cfg(test)]
    pub fn with_manifest_loader(loader: M) -> Self {
        Self {
            manifest_loader: loader,
            cache: Default::default(),
        }
    }
}

impl<M> Loader for FileLoader<M>
where
    M: ManifestLoader,
{
    /// Loads a [`Manifest`] from the specified path.
    ///
    /// Checks the cache for a previously loaded [`Manifest`]. If found, returns a reference to the
    /// cached manifest. Otherwise, loads the manifest, caches it, and returns a reference to the
    /// newly loaded manifest.
    ///
    /// Returns an `Arc`-wrapped [`Manifest`] on success, or an error if loading fails.
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Arc<Manifest>, Error> {
        let path = path.as_ref().to_owned();

        {
            let cache = self.cache.read();
            if let Some(cached) = cache.get(&path) {
                return Ok(cached.clone());
            }
        }

        let manifest = self.manifest_loader.load_manifest(&path).map(Arc::new)?;

        {
            let mut cache = self.cache.write();
            cache.insert(path, manifest.clone());
        }

        Ok(manifest)
    }
}

/// Internal trait for loading manifests from a specific source.
pub trait ManifestLoader {
    /// Loads a [`Manifest`] from the specified path.
    ///
    /// Returns a [`Manifest`] on success, or an error if loading fails.
    fn load_manifest(&self, path: &Path) -> Result<Manifest, Error>;
}

/// Internal implementation of the [`ManifestLoader`] trait for loading manifests from the filesystem.
#[derive(Debug, Clone)]
pub struct ManifestFileLoader {
    root: Box<Path>,
}

impl ManifestFileLoader {
    fn new(root: impl Into<Box<Path>>) -> Self {
        Self { root: root.into() }
    }
}

impl ManifestLoader for ManifestFileLoader {
    fn load_manifest(&self, path: &Path) -> Result<Manifest, Error> {
        let full_path = self.root.join(path);

        // Check if the path exists
        if !full_path.exists() {
            return Err(Error::FileOpen {
                path: full_path.clone(),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "Path does not exist"),
            });
        }

        // Determine the actual file path to read
        let manifest_path = if full_path.is_dir() {
            full_path.join("package.toml")
        } else {
            full_path
        };

        // Check if the file exists
        if !manifest_path.exists() {
            return Err(Error::FileOpen {
                path: manifest_path.clone(),
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "File does not exist"),
            });
        }

        let mut file = File::open(&manifest_path)
            .map(BufReader::new)
            .map_err(|err| Error::FileOpen {
                path: manifest_path.clone(),
                source: err,
            })?;

        let mut manifest_content = String::with_capacity(2048);
        file.read_to_string(&mut manifest_content)
            .map_err(|err| Error::FileRead {
                path: manifest_path.clone(),
                source: err,
            })?;

        let manifest =
            crate::manifest::from_str(&manifest_content).map_err(|err| Error::ManifestParse {
                path: manifest_path,
                source: err,
            })?;

        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use super::{Error, FileLoader, Loader, ManifestLoader};
    use crate::manifest::{Manifest, PackageMeta};

    mockall::mock! {
        /// A mock implementation of the [`ManifestLoader`] trait
        ManifestLoader {}

        impl ManifestLoader for ManifestLoader {
            fn load_manifest(&self, path: &Path) -> Result<Manifest, Error>;
        }
    }

    #[test]
    fn cache_hits() {
        //* Given
        let path = PathBuf::from("./contracts/package.toml");

        // Create a minimal valid manifest
        let manifest = Manifest {
            package: PackageMeta::new("contracts".parse().expect("Failed to parse package name")),
            service: None,
            init: vec![],
        };

        // Construct the loader with a mock manifest loader
        let loader = {
            let expected_path = path.clone();

            let mut mock_loader = MockManifestLoader::new();
            mock_loader
                .expect_load_manifest()
                .withf(move |p: &Path| p == expected_path.as_path())
                .times(1)
                .returning(move |_| Ok(manifest.clone()));

            FileLoader::with_manifest_loader(mock_loader)
        };

        // Pre-load the manifest
        let manifest1 = loader.load(&path).expect("Failed to pre-load manifest");

        //* When
        let manifest2 = loader.load(&path).expect("Failed to load manifest");

        //* Then
        assert!(
            Arc::ptr_eq(&manifest1, &manifest2),
            "Should return the same manifest instance"
        );
    }

    #[test]
    fn different_paths_dont_share_cache() {
        //* Given
        let path1 = PathBuf::from("./contracts/package.toml");
        let path2 = PathBuf::from("./db/package.toml");

        // Create minimal valid manifests
        let manifest1 = Manifest {
            package: PackageMeta::new("contracts".parse().expect("Failed to parse package name")),
            service: None,
            init: vec![],
        };

        let manifest2 = Manifest {
            package: PackageMeta::new("db".parse().expect("Failed to parse package name")),
            service: None,
            init: vec![],
        };

        // Construct the loader with a mock manifest loader
        let loader = {
            let expected_path1 = path1.clone();
            let expected_path2 = path2.clone();

            let mut mock_loader = MockManifestLoader::new();
            mock_loader
                .expect_load_manifest()
                .withf(move |p: &Path| p == expected_path1.as_path())
                .times(1)
                .returning(move |_| Ok(manifest1.clone()));

            mock_loader
                .expect_load_manifest()
                .withf(move |p: &Path| p == expected_path2.as_path())
                .times(1)
                .returning(move |_| Ok(manifest2.clone()));

            FileLoader::with_manifest_loader(mock_loader)
        };

        //* When
        let result1 = loader.load(&path1).expect("Failed to load manifest 1");
        let result2 = loader.load(&path2).expect("Failed to load manifest 2");

        //* Then
        assert!(
            !Arc::ptr_eq(&result1, &result2),
            "Should return different manifest instances"
        );
    }

    #[test]
    fn manifest_load_file_open_error_handling() {
        //* Given
        let path = PathBuf::from("./contracts/package.toml");

        // Mock the manifest loader
        let loader = {
            let expected_path = path.clone();

            let mut mock_loader = MockManifestLoader::new();
            mock_loader
                .expect_load_manifest()
                .withf(move |p: &Path| p == expected_path.as_path())
                .times(2)
                .returning(|_| {
                    Err(Error::FileOpen {
                        path: PathBuf::from("./contracts/package.toml"),
                        source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
                    })
                });

            FileLoader::with_manifest_loader(mock_loader)
        };

        //* When
        let result1 = loader.load(&path);
        let result2 = loader.load(&path);

        //* Then
        assert!(result1.is_err(), "First load should fail");
        assert!(result2.is_err(), "Second load should fail");
    }
}
