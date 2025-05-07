use std::{
    fs::File,
    path::{Path, PathBuf},
};

use seahaven_cli::result::Result;
use seahaven_compose_file::ComposeFile;
use seahaven_setup_file::Env;

/// Creates a new temporary directory where the `.env` and `docker-compose.yaml`
/// files will be written
pub fn new() -> Uninitialized {
    Uninitialized { _priv: () }
}

/// Represents the initial state before any directory is created
pub struct Uninitialized {
    /// Private field to prevent instantiation
    _priv: (),
}

impl Uninitialized {
    /// Creates a new temporary directory
    pub fn create_dir(self) -> Result<DirCreated> {
        let dir =
            create_temp_dir().map_err(|err| anyhow::anyhow!("Failed to create tempdir: {err}"))?;
        tracing::debug!("Created temporary directory: {}", dir.path().display());

        Ok(DirCreated { dir })
    }
}

/// Represents a state where only the directory has been created
pub struct DirCreated {
    dir: tempfile::TempDir,
}

impl DirCreated {
    /// Writes the environment variables to the .env file
    pub fn write_env_file(self, env: &Env) -> Result<EnvFileOnly> {
        let env_file_path = self.dir.path().join(".env");
        tracing::debug!("Writing .env file: {}", env_file_path.display());
        write_env_file(&env_file_path, env)
            .map_err(|err| anyhow::anyhow!("Failed to create .env file: {err}"))?;
        tracing::debug!("Wrote .env file: {}", env_file_path.display());
        Ok(EnvFileOnly {
            dir: self.dir,
            env_file_path,
        })
    }

    /// Writes both the environment variables and Docker Compose configuration
    pub fn write_all(self, env: &Env, compose: &ComposeFile) -> Result<Complete> {
        let compose_file_path = self.dir.path().join("docker-compose.yaml");
        tracing::debug!(
            "Writing docker-compose.yaml file: {}",
            compose_file_path.display()
        );
        write_compose_file(&compose_file_path, compose)
            .map_err(|err| anyhow::anyhow!("Failed to write docker-compose.yaml file: {err}"))?;
        tracing::debug!(
            "Wrote docker-compose.yaml file: {}",
            compose_file_path.display()
        );

        let env_file_path = self.dir.path().join(".env");
        tracing::debug!("Writing .env file: {}", env_file_path.display());
        write_env_file(&env_file_path, env)
            .map_err(|err| anyhow::anyhow!("Failed to write .env file: {err}"))?;
        tracing::debug!("Wrote .env file: {}", env_file_path.display());

        Ok(Complete {
            dir: self.dir,
            env_file_path,
            compose_file_path,
        })
    }
}

impl HasTempDirPath for DirCreated {
    fn temp_dir_path(&self) -> &Path {
        self.dir.path()
    }
}

/// Represents a state where directory and `.env` file have been created.
///
/// This is a terminal state.
pub struct EnvFileOnly {
    dir: tempfile::TempDir,
    env_file_path: PathBuf,
}

impl HasTempDirPath for EnvFileOnly {
    fn temp_dir_path(&self) -> &Path {
        self.dir.path()
    }
}

impl HasEnvFilePath for EnvFileOnly {
    fn env_file_path(&self) -> &Path {
        self.env_file_path.as_path()
    }
}

/// Represents a state where directory, `.env` file, and `docker-compose.yaml` file have been created
///
/// This is a terminal state.
pub struct Complete {
    dir: tempfile::TempDir,
    env_file_path: PathBuf,
    compose_file_path: PathBuf,
}

impl HasTempDirPath for Complete {
    fn temp_dir_path(&self) -> &Path {
        self.dir.path()
    }
}

impl HasEnvFilePath for Complete {
    fn env_file_path(&self) -> &Path {
        self.env_file_path.as_path()
    }
}

impl HasComposeFilePath for Complete {
    fn compose_file_path(&self) -> &Path {
        self.compose_file_path.as_path()
    }
}

/// Helper trait to get the temporary directory path
pub trait HasTempDirPath {
    fn temp_dir_path(&self) -> &Path;
}

/// Helper trait to get the `.env` file path
pub trait HasEnvFilePath {
    fn env_file_path(&self) -> &Path;
}

/// Helper trait to get the `docker-compose.yaml` file path
pub trait HasComposeFilePath {
    fn compose_file_path(&self) -> &Path;
}

fn create_temp_dir() -> anyhow::Result<tempfile::TempDir> {
    let dir = tempfile::Builder::new().prefix("seahaven-").tempdir()?;
    Ok(dir)
}

fn write_env_file(path: &Path, env: &Env) -> anyhow::Result<()> {
    let env_file = File::create(path)?;
    serde_envfile::to_writer(env_file, env)?;
    Ok(())
}

fn write_compose_file(path: &Path, compose: &ComposeFile) -> anyhow::Result<()> {
    let compose_file = File::create(path)?;
    seahaven_compose_file::ser::to_writer(compose_file, compose)?;
    Ok(())
}
