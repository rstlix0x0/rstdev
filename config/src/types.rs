use rst_common::with_errors::thiserror::Error;

use crate::format::Format;

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("unable to load format: {0}")]
    FormatError(String),

    #[error("unable to parse configuration: {0}")]
    ParseError(String),
}

/// SourceParser is a public interface / trait that must be
/// implemented by any adapters that need to parse config as
/// a string from some source, like file, env vars or others
pub trait SourceParser {
    fn fetch(&self) -> Result<Format, ConfigError>;
}
