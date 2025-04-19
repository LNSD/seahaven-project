//! # Seahaven file processing

use std::{fs::File, io::BufReader, path::Path};

use seahaven_cli::result::Result;
use seahaven_file::{Content, Env};

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

    let res = seahaven_file::from_reader(file)
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
