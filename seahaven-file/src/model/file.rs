use super::{content::Content, env::Env};

/// A setup file containing an optional environment configuration and content.
///
/// The `SetupFile` struct represents a configuration file that can optionally include
/// environment-specific settings along with its main content. This allows for flexible
/// configuration management across different environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupFile(Option<Env>, Content);

impl SetupFile {
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

    /// Consumes the `SetupFile` and returns its components as a tuple.
    ///
    /// Returns a tuple containing:
    /// - An optional environment configuration (`Option<Env>`)
    /// - The main configuration content (`Content`)
    pub fn unpack(self) -> (Option<Env>, Content) {
        (self.0, self.1)
    }
}

impl From<(Option<Env>, Content)> for SetupFile {
    /// Creates a new setup file from an environment and content tuple.
    ///
    /// Returns a new [`SetupFile`] instance containing the provided environment and content.
    fn from((env, content): (Option<Env>, Content)) -> Self {
        Self(env, content)
    }
}

impl From<Content> for SetupFile {
    /// Creates a new setup file from content only, with no environment configuration.
    ///
    /// Returns a new [`SetupFile`] instance containing only the provided content, with no environment
    /// configuration (None).
    fn from(content: Content) -> Self {
        Self(None, content)
    }
}
