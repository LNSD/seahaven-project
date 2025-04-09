use std::io::{BufRead, Seek};

use crate::{
    matter,
    model::{
        SetupFile, content,
        env::{self, Env},
    },
};

/// Parses a Seahaven setup description YAML file from a IO stream
///
/// This function extracts and parses both the front-matter section (if present) and the main YAML content.
/// The front-matter is parsed into an optional [`Env`] containing environment variables,
/// while the remaining content is parsed into a [`Content`](content::Content).
///
/// See [`ParsingError`] for the errors that can occur when parsing a setup description file
pub fn from_reader<R>(reader: R) -> Result<SetupFile, ParsingError>
where
    R: BufRead + Seek,
{
    let (front_matter, content) = matter::extract_front_matter(reader)?;

    let envfile = front_matter.map(env::de::from_reader).transpose()?;
    let file_content = content::de::from_reader(content)?;

    Ok(SetupFile::from((envfile, file_content)))
}

/// Parses a Seahaven setup description file from a IO stream and returns the envfile.
///
/// This function extracts and parses the front-matter section (if present) and returns
/// the environment variables as an [`Env`].
///
/// See [`ParsingError`] for the errors that can occur when parsing a setup description file
pub fn fileenv_from_reader<R>(reader: R) -> Result<Option<Env>, ParsingError>
where
    R: BufRead + Seek,
{
    let (front_matter, _) = matter::extract_front_matter(reader)?;

    let env_file = front_matter.map(env::de::from_reader).transpose()?;

    Ok(env_file)
}

/// Errors that can occur when parsing a setup description file
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ParsingError {
    /// The front matter section of a file has an invalid format
    #[error("invalid front-matter format: {0}")]
    InvalidFrontMatterFormat(#[from] matter::Error),

    /// The environment variables in the front-matter section cannot be parsed
    #[error("environment variables parsing failed: {0}")]
    EnvParsingFailed(#[from] env::de::DeserializationError),

    /// The main content of the setup description file cannot be parsed
    #[error("content deserialization failed: {0}")]
    ContentDeserializationFailed(#[from] content::de::DeserializationError),
}
