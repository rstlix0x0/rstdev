use rst_common::with_errors::thiserror::Error;

use crate::format::Format;

/// ConfigError is a custom internal error that will be used
/// when parsing of fetching data format
///
/// There are two enum keys:
///
/// - [`ConfigError::FormatError`]
/// - [`ConfigError::ParseError`]
#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    /// `FormatError` will be used when we're unable to read original format
    #[error("unable to load format: {0}")]
    FormatError(String),

    /// `ParseError` will be used when we're unable to read configuration from some sources
    #[error("unable to parse configuration: {0}")]
    ParseError(String),
}

/// SourceParser is a public interface / trait that must be
/// implemented by any adapters that need to parse config as
/// a string from some source, like file, env vars or others
pub trait SourceParser {
    fn fetch(&self) -> Result<Format, ConfigError>;
}
