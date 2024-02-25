use std::marker::PhantomData;

use rst_common::standard::serde::de::DeserializeOwned;

use crate::types::{ConfigError, SourceFormatter};

/// Format is a object wrapper of given input as a string value
///
/// This struct object already been refactored to accept any input source
/// that implement [`SourceFormatter`]
///
/// Final implementation of this object is removing all specific source type
/// implementations like `as_toml`, or `as_yaml`, and only provide a single method
/// which is [`Source::parse`] that will parse given source type. This final approach
/// designed to make this object become modular, by separating between the object with
/// its specific type parser.
///
/// By using this approach, user will be able to create a custom source type parser based on their needs 
pub struct Source<TFormatter, TValue>
where
    TValue: Clone,
    TFormatter: for<'a> SourceFormatter<'a, TValue>,
{
    input: TFormatter,
    _phantomf: Option<PhantomData<TValue>>,
}

impl<T, St> Source<T, St>
where
    St: Clone,
    T: for<'a> SourceFormatter<'a, St>,
{
    pub fn new(input: T) -> Self {
        Self {
            input,
            _phantomf: None,
        }
    }

    pub fn parse<F, Out>(&self, cb: F) -> Result<Out, ConfigError>
    where
        F: FnOnce(St) -> Result<Out, ConfigError>,
        Out: DeserializeOwned,
    {
        cb(self.input.get_source_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;
    use toml;

    use rst_common::standard::serde::{self, Deserialize, Serialize};
    use rst_common::standard::serde_json;

    use crate::format::{from_toml, from_yaml, from_json};
    use crate::values::StringValue;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "self::serde")]
    struct Message {
        msg: String,
    }

    #[test]
    fn test_parse_toml() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let toml_str = toml::to_string(&input);
        assert!(!toml_str.is_err());

        let input_value = StringValue::new(toml_str.unwrap());
        let source = Source::new(input_value);
        let out: Result<Message, ConfigError> = source.parse(from_toml);
        assert!(!out.is_err());
        assert_eq!("hello world".to_string(), out.unwrap().msg)
    }

    #[test]
    fn test_parse_yaml() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let yaml_str = serde_yaml::to_string(&input);
        assert!(!yaml_str.is_err());

        let input_value = StringValue::new(yaml_str.unwrap());
        let source = Source::new(input_value);
        let out: Result<Message, ConfigError> = source.parse(from_yaml);
        assert!(!out.is_err());
        assert_eq!("hello world".to_string(), out.unwrap().msg)
    }

    #[test]
    fn test_parse_json() {
        let input = Message {
            msg: "hello world".to_string(),
        };

        let json_str = serde_json::to_string(&input);
        assert!(!json_str.is_err());

        let input_value = StringValue::new(json_str.unwrap());
        let source = Source::new(input_value);
        let out: Result<Message, ConfigError> = source.parse(from_json);
        assert!(!out.is_err());
        assert_eq!("hello world".to_string(), out.unwrap().msg)
    }
}
