use serde_yaml;

use rst_common::standard::serde::Deserialize;

use crate::types::ConfigError;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rst_common::standard::serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Message {
        msg: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
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
}
