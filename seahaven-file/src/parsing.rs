use std::io::{BufRead, Seek};

use crate::{
    env::{self, Env},
    matter,
    model::{self, Content},
};

/// A setup file containing an optional environment configuration and content.
///
/// The `File` struct represents a configuration file that can optionally include
/// environment-specific settings along with its main content. This allows for flexible
/// configuration management across different environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File(Option<Env>, Content);

impl File {
    /// Returns a reference to the environment configuration if present.
    ///
    /// The environment configuration contains variables and settings defined in the front matter
    /// section of the setup file. Returns `None` if no environment configuration exists.
    pub fn env(&self) -> Option<&Env> {
        self.0.as_ref()
    }

    /// Returns a reference to the main configuration content.
    ///
    /// The content contains the core setup configuration including Docker Compose elements
    /// like services, networks, volumes, configs, and secrets.
    pub fn content(&self) -> &Content {
        &self.1
    }

    /// Returns a mutable reference to the environment configuration if present.
    ///
    /// Allows modifying the environment variables and settings in the front matter.
    /// Returns `None` if no environment configuration exists.
    pub fn env_mut(&mut self) -> Option<&mut Env> {
        self.0.as_mut()
    }

    /// Returns a mutable reference to the main configuration content.
    ///
    /// Allows modifying the core setup configuration including Docker Compose elements
    /// like services, networks, volumes, configs, and secrets.
    pub fn content_mut(&mut self) -> &mut Content {
        &mut self.1
    }

    /// Consumes the `File` and returns its components as a tuple.
    ///
    /// Returns a tuple containing:
    /// - An optional environment configuration (`Option<Env>`)
    /// - The main configuration content (`Content`)
    pub fn unpack(self) -> (Option<Env>, Content) {
        (self.0, self.1)
    }
}

impl From<(Option<Env>, Content)> for File {
    /// Creates a new setup file from an environment and content tuple.
    ///
    /// Returns a new [`File`] instance containing the provided environment and content.
    fn from((env, content): (Option<Env>, Content)) -> Self {
        Self(env, content)
    }
}

impl From<Content> for File {
    /// Creates a new setup file from content only, with no environment configuration.
    ///
    /// Returns a new [`File`] instance containing only the provided content, with no environment
    /// configuration (None).
    fn from(content: Content) -> Self {
        Self(None, content)
    }
}

/// Parses a Seahaven setup description YAML file from a IO stream
///
/// This function extracts and parses both the front-matter section (if present) and the main YAML content.
/// The front-matter is parsed into an optional [`Env`] containing environment variables,
/// while the remaining content is parsed into a [`Content`](content::Content).
///
/// See [`ParsingError`] for the errors that can occur when parsing a setup description file
pub fn from_reader<R>(reader: R) -> Result<File, ParsingError>
where
    R: BufRead + Seek,
{
    let (front_matter, content) = matter::extract_front_matter(reader)?;

    let envfile = front_matter.map(env::de::from_reader).transpose()?;
    let file_content = model::parsing::from_reader(content)?;

    Ok(File::from((envfile, file_content)))
}

/// Parses a Seahaven setup description file from a IO stream and returns the envfile.
///
/// This function extracts and parses the front-matter section (if present) and returns
/// the environment variables as an [`Env`].
///
/// See [`ParsingError`] for the errors that can occur when parsing a setup description file
pub fn env_from_reader<R>(reader: R) -> Result<Option<env::Env>, ParsingError>
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
    #[error("content parsing failed: {0}")]
    ContentParsingFailed(#[from] model::parsing::Error),
}
