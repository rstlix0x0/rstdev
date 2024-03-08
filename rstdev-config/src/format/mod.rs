//! A `format` module used to store multiple config data source type parsers
//!
//! By default this module already provide an implementation source type parser for:
//! - `TOML`
//! - `YAML`
//! - `JSON`
//! - Environment Variables
//!
//! All given config source MUST BE implement `serde::de::DeserializeOwned`
use envy;
use serde_yaml;
use toml;

use rst_common::standard::serde::de::DeserializeOwned;
use rst_common::standard::serde_json;

use crate::types::ConfigError;

pub fn use_toml<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_toml = toml::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_toml)
}

pub fn use_yaml<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_yaml = serde_yaml::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_yaml)
}

pub fn use_json<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_json = serde_json::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_json)
}

pub fn use_env<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: IntoIterator<Item = (String, String)>,
    Out: DeserializeOwned,
{
    let from_env =
        envy::from_iter(input).map_err(|err| ConfigError::FormatError(err.to_string()))?;
    Ok(from_env)
}
