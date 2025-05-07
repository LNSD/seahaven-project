use std::{fs::File, io::BufReader, path::Path};

use seahaven_cli::result::Result;
use seahaven_setup_file::Env;

/// Loads the environment variables from a setup file.
///
/// Returns an error if the file cannot be opened or parsed.
pub fn load_setup_file_env(path: &impl AsRef<Path>) -> Result<Option<Env>> {
    let file = File::open(path).map(BufReader::new).map_err(|err| {
        anyhow::anyhow!(
            "Failed to open setup file '{}': {}",
            path.as_ref().display(),
            err
        )
    })?;

    let env = seahaven_setup_file::env_from_reader(file).map_err(|err| {
        anyhow::anyhow!(
            "Failed to parse setup file '{}': {}",
            path.as_ref().display(),
            err
        )
    })?;

    Ok(env)
}

/// Loads environment variables from files, with fallback to project directory's `.env` file.
///
/// If no environment files are provided, it will look for a `.env` file in the project directory.
/// If found, it will be used. If not found, an empty environment will be returned.
///
/// The environment variables from the files are merged with the front matter environment.
pub fn load_and_merge_envs(
    file_paths: Option<impl IntoIterator<Item = impl AsRef<Path>>>,
    project_directory: &impl AsRef<Path>,
    front_matter_env: Option<Env>,
) -> Result<Env> {
    let paths = file_paths
        .map(|paths| {
            paths
                .into_iter()
                .map(|p| p.as_ref().to_path_buf())
                .collect()
        })
        .unwrap_or_else(|| {
            let env_file_path = project_directory.as_ref().join(".env");
            if env_file_path.exists() {
                tracing::debug!(
                    "Found .env file in project directory: {}",
                    env_file_path.display()
                );
                vec![env_file_path]
            } else {
                tracing::debug!("No .env file found in project directory, skipping");
                vec![]
            }
        });

    let files_env = load_files(paths)?;

    // Merge the files env with the front matter env
    let env = match (files_env, front_matter_env) {
        (files_env, None) => files_env,
        (files_env, Some(front_matter_env)) => {
            let mut env = files_env;
            env.extend(front_matter_env);
            env
        }
    };

    Ok(env)
}

/// Loads all variables found in the files into the environment,
/// overriding any existing environment variables of the same name.
///
/// If a variable is specified multiple times in different files,
/// then the last occurrence is applied.
///
/// Files are loaded in order, with later values overriding earlier ones.
/// If a file is invalid or unreadable, an error is returned.
fn load_files<P>(files: impl IntoIterator<Item = P>) -> Result<Env>
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

    Ok(env)
}
