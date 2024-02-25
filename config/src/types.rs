use rst_common::{
    standard::serde::{de::DeserializeOwned, Deserialize},
    with_errors::thiserror::{self, Error},
};

use crate::source::Source;

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

/// ConfigFormatter is a public interface / trait that must be implemented
/// by all source value.
///
/// Before this trait exists, the formatter value by default is `String`.
/// The problem begins when we're try to use other format like `std::env::vars()`
/// that give us an iterator of key and value in string.
///
/// That's why rather than depends on single hardcoded value (`String`), it will be better to design the config
/// value itself based on this trait. Each of value must be implement this trait and provide
/// all source types including `as_env`.
pub trait ConfigFormatter<'a> {
    fn as_yaml<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: Deserialize<'a>;
    fn as_toml<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned;
    fn as_json<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned;
    fn as_env<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned;
}

/// SourceParser is a public interface / trait that must be
/// implemented by any adapters that need to parse config as
/// a string from some source, like file, env vars or others
pub trait SourceParser<T>
where
    T: for<'a> ConfigFormatter<'a>,
{
    fn fetch(&self) -> Result<Source<T>, ConfigError>;
}
