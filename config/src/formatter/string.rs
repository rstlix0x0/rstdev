use serde_yaml;
use toml;

use rst_common::standard::serde::{de::DeserializeOwned, Deserialize};
use rst_common::standard::serde_json;

use crate::types::{ConfigError, ConfigFormatter};

/// `Value` is a main object that will be used as
/// value type of `String`.
///
/// This object MUST implement [`ConfigFormatter`]
pub struct Value {
    input: String,
}

impl Value {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl<'a> ConfigFormatter<'a> for Value {
    fn as_json<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned,
    {
        let from_json = serde_json::from_str(&self.input)
            .map_err(|err| ConfigError::FormatError(err.to_string()));
        Some(from_json)
    }

    fn as_toml<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned,
    {
        let from_toml =
            toml::from_str(&self.input).map_err(|err| ConfigError::FormatError(err.to_string()));
        Some(from_toml)
    }

    fn as_yaml<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: Deserialize<'a>,
    {
        let from_yaml = serde_yaml::from_slice(self.input.as_bytes())
            .map_err(|err| ConfigError::FormatError(err.to_string()));

        Some(from_yaml)
    }

    fn as_env<T>(&'a self) -> Option<Result<T, ConfigError>>
    where
        T: DeserializeOwned,
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rst_common::standard::serde::{self, Deserialize, Serialize};
    use serde_yaml;

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

        let value = Value::new(str.unwrap());
        let output: Option<Result<Message, ConfigError>> = value.as_yaml();
        assert!(output.is_some());

        let res = output.unwrap();
        assert!(!res.is_err());
        assert_eq!("hello world".to_string(), res.unwrap().msg)
    }

    #[test]
    fn test_fetch_invalid_as_yaml() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_yaml::to_string(&msg);
        assert!(!str.is_err());

        let format = Value::new(str.unwrap());
        let output: Option<Result<MessageInvalid, ConfigError>> = format.as_yaml();
        assert!(output.is_some());
        assert!(matches!(
            output.unwrap().unwrap_err(),
            ConfigError::FormatError(_)
        ))
    }

    #[test]
    fn test_fetch_as_toml() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let toml_str = toml::to_string(&input);
        assert!(!toml_str.is_err());

        let format = Value::new(toml_str.unwrap());
        let output: Option<Result<Message, ConfigError>> = format.as_toml();
        assert!(output.is_some());
        assert_eq!("hello world".to_string(), output.unwrap().unwrap().msg)
    }

    #[test]
    fn test_fetch_invalid_as_toml() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_yaml::to_string(&msg);
        assert!(!str.is_err());

        let format = Value::new(str.unwrap());
        let output: Option<Result<MessageInvalid, ConfigError>> = format.as_toml();
        assert!(output.is_some());
        assert!(matches!(
            output.unwrap().unwrap_err(),
            ConfigError::FormatError(_)
        ))
    }

    #[test]
    fn test_fetch_as_json() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let json_str = serde_json::to_string(&input);
        assert!(!json_str.is_err());

        let format = Value::new(json_str.unwrap());
        let output: Option<Result<Message, ConfigError>> = format.as_json();
        assert!(output.is_some());
        assert_eq!(output.unwrap().unwrap().msg, "hello world".to_string())
    }

    #[test]
    fn test_fetch_invalid_as_json() {
        let msg = Message {
            msg: "hello world".to_string(),
        };
        let str = serde_json::to_string(&msg);
        assert!(!str.is_err());

        let format = Value::new(str.unwrap());
        let output: Option<Result<MessageInvalid, ConfigError>> = format.as_json();
        assert!(output.is_some());
        assert!(matches!(
            output.unwrap().unwrap_err(),
            ConfigError::FormatError(_)
        ))
    }
}
