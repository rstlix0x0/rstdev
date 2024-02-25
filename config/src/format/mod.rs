//! A `format` module used to store multiple config data source type parsers 
//!
//! By default this module already provide an implementation source type parser for: 
//! - `TOML`
//! - `YAML`
//! - `JSON`
//! 
//! All given config source MUST BE implement `serde::de::DeserializeOwned`
use serde_yaml;
use toml;

use rst_common::standard::serde::de::DeserializeOwned;
use rst_common::standard::serde_json;

use crate::types::ConfigError;

pub fn from_toml<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_toml = toml::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_toml)
}

pub fn from_yaml<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_yaml = serde_yaml::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_yaml)
}

pub fn from_json<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_json = serde_json::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_json)
}
