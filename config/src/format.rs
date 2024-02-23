use serde_yaml;
use toml;

use rst_common::standard::serde::{de::DeserializeOwned, Deserialize};
use rst_common::standard::serde_json;

use crate::types::ConfigError;

/// Format is a object wrapper of given input as a string value
///
/// This object used to parse given string based on their original format.
/// Available format:
///
/// - YAML
/// - TOML
/// - JSON
pub struct Format {
    input: String,
}

impl<'a> Format {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn as_yaml<T>(&'a self) -> Result<T, ConfigError>
    where
        T: Deserialize<'a>,
    {
        serde_yaml::from_slice(self.input.as_bytes())
            .map_err(|err| ConfigError::FormatError(err.to_string()))
    }

    pub fn as_toml<T>(&'a self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
    {
        toml::from_str(&self.input).map_err(|err| ConfigError::FormatError(err.to_string()))
    }

    pub fn as_json<T>(&'a self) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(&self.input).map_err(|err| ConfigError::FormatError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rst_common::standard::serde::{self, Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "self::serde")]
    struct Message {
        msg: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "self::serde")]
    struct MessageInvalid {
        msg2: String,
    }

    #[test]
    fn test_fetch_as_yaml() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_yaml::to_string(&msg);
        assert!(!str.is_err());

        let format = Format::new(str.unwrap());
        let output: Result<Message, ConfigError> = format.as_yaml();
        assert!(!output.is_err());
        assert_eq!("hello world".to_string(), output.unwrap().msg)
    }

    #[test]
    fn test_fetch_invalid_as_yaml() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_yaml::to_string(&msg);
        assert!(!str.is_err());

        let format = Format::new(str.unwrap());
        let output: Result<MessageInvalid, ConfigError> = format.as_yaml();
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), ConfigError::FormatError(_)))
    }

    #[test]
    fn test_fetch_as_toml() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let toml_str = toml::to_string(&input);
        assert!(!toml_str.is_err());

        let format = Format::new(toml_str.unwrap());
        let output: Result<Message, ConfigError> = format.as_toml();
        assert!(!output.is_err());
        assert_eq!("hello world".to_string(), output.unwrap().msg)
    }

    #[test]
    fn test_fetch_invalid_as_toml() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_yaml::to_string(&msg);
        assert!(!str.is_err());

        let format = Format::new(str.unwrap());
        let output: Result<MessageInvalid, ConfigError> = format.as_toml();
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), ConfigError::FormatError(_)))
    }

    #[test]
    fn test_fetch_as_json() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let json_str = serde_json::to_string(&input);
        assert!(!json_str.is_err());

        let format = Format::new(json_str.unwrap());
        let output: Result<Message, ConfigError> = format.as_json();
        assert!(!output.is_err());
    }

    #[test]
    fn test_fetch_invalid_as_json() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_json::to_string(&msg);
        assert!(!str.is_err());

        let format = Format::new(str.unwrap());
        let output: Result<MessageInvalid, ConfigError> = format.as_json();
        assert!(output.is_err());
        assert!(matches!(output.unwrap_err(), ConfigError::FormatError(_)))
    }
}
