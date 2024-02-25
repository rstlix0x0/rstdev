use rst_common::standard::serde::{de::DeserializeOwned, Deserialize};

use crate::types::{ConfigError, ConfigFormatter};

/// Format is a object wrapper of given input as a string value
///
/// This object used to parse given string based on their original format.
/// Available format:
///
/// - YAML
/// - TOML
/// - JSON
/// - Environment Variables
///
/// This struct object already been refactored to accept any input source
/// that implement [`ConfigFormatter`]
pub struct Source<T>
where
    T: for<'a> ConfigFormatter<'a>,
{
    input: T,
}

impl<T> Source<T>
where
    T: for<'a> ConfigFormatter<'a>,
{
    pub fn new(input: T) -> Self {
        Self { input }
    }

    pub fn as_yaml<'b, Out>(&'b self) -> Option<Result<Out, ConfigError>>
    where
        Out: Deserialize<'b>,
    {
        self.input.as_yaml()
    }

    pub fn as_toml<'b, Out>(&'b self) -> Option<Result<Out, ConfigError>>
    where
        Out: DeserializeOwned,
    {
        self.input.as_toml()
    }

    pub fn as_json<'b, Out>(&'b self) -> Option<Result<Out, ConfigError>>
    where
        Out: DeserializeOwned,
    {
        self.input.as_json()
    }

    pub fn as_env<'b, Out>(&'b self) -> Option<Result<Out, ConfigError>>
    where
        Out: DeserializeOwned,
    {
        self.input.as_env()
    }
}
