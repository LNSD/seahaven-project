use super::parts::{DeserializedRoot, ValidatedRoot as Root};

/// Deserialize and validate a setup file content from a string
pub fn from_str(s: &str) -> Result<Root, Error> {
    let deserialized = serde_yaml::from_str(s).map_err(Error::ParsingError)?;
    validate_root(deserialized).map_err(Error::ValidationError)
}

/// Deserialize and validate a setup file content from a reader
pub fn from_reader<R>(reader: R) -> Result<Root, Error>
where
    R: std::io::Read,
{
    let deserialized = serde_yaml::from_reader(reader).map_err(Error::ParsingError)?;
    validate_root(deserialized).map_err(Error::ValidationError)
}

/// An error that happened when parsing and validating the setup file content
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Setup file deserialization failed
    #[error(transparent)]
    ParsingError(#[from] serde_yaml::Error),

    /// Setup file validation failed
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

/// An error that happened when validating the setup file content
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ValidationError(#[from] Box<dyn std::error::Error + Send + Sync>);

/// Validate the setup file content
fn validate_root(parsed: DeserializedRoot) -> Result<Root, ValidationError> {
    // There MUST be at least one service
    if parsed.services.is_empty() {
        return Err(ValidationError(
            anyhow::anyhow!("There must be at least one service").into(),
        ));
    }

    // There MUST be no name overlap between `services` and `init` keys
    if let Some(init) = &parsed.init {
        let bad_keys = init
            .keys()
            .filter(|key| parsed.services.contains_key(*key))
            .collect::<Vec<_>>();
        if !bad_keys.is_empty() {
            return Err(ValidationError(
                anyhow::anyhow!("The following names overlap: {:?}", bad_keys).into(),
            ));
        }
    }

    Ok(Root {
        name: parsed.name,
        services: parsed.services,
        init: parsed.init,
        _rest: parsed._rest,
    })
}
