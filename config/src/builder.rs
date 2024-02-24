use std::marker::PhantomData;

use crate::types::{ConfigError, ConfigFormatter, SourceParser};
use crate::Format;

/// Builder is a main object used to manage multiple source parser
///
/// This object will depends on [`SourceParser`] adapter which have to
/// be implemented by all adapters.  Available adapters:
///
/// - File
///
/// Example:
///
/// ```rust
/// let cfg: Message = Builder::new(from_file(yaml_file))
///  .fetch()?
///  .as_yaml()?;
/// ```
pub struct Builder<T, F>
where
    F: for<'a> ConfigFormatter<'a>,
    T: for<'a> SourceParser<F>,
{
    adapter: T,
    _phantomf: Option<PhantomData<F>>,
}

impl<T, F> Builder<T, F>
where
    F: for<'a> ConfigFormatter<'a>,
    T: for<'a> SourceParser<F>,
{
    pub fn new(adapter: T) -> Self {
        Self {
            adapter,
            _phantomf: None,
        }
    }

    pub fn fetch(&self) -> Result<Format<F>, ConfigError> {
        self.adapter.fetch()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::from_file;
    use std::path::PathBuf;

    use rst_common::standard::serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(crate = "self::serde")]
    struct Message {
        message: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(crate = "self::serde")]
    struct MessageGroup {
        message: String,
        keys: MessageGroupKeys,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(crate = "self::serde")]
    struct MessageGroupKeys {
        key1: String,
        key2: String,
    }

    #[test]
    fn test_parse_config_yaml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let yaml_file = format!("{}/test.yaml", path.display());
        let cfg: Option<Result<Message, ConfigError>> =
            Builder::new(from_file(yaml_file)).fetch()?.as_yaml();

        assert_eq!(cfg.unwrap().unwrap().message, "hello world");
        Ok(())
    }

    #[test]
    fn test_parse_config_toml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let toml_file = format!("{}/test.toml", path.display());
        let cfg: Option<Result<MessageGroup, ConfigError>> =
            Builder::new(from_file(toml_file)).fetch()?.as_toml();

        let resp = cfg.unwrap().unwrap();
        assert_eq!(resp.clone().message, "hello world");
        assert_eq!(resp.clone().keys.key1, "value1");
        assert_eq!(resp.clone().keys.key2, "value2");
        Ok(())
    }

    #[test]
    fn test_parse_config_json() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let json_file = format!("{}/test.json", path.display());
        let cfg: Option<Result<MessageGroup, ConfigError>> =
            Builder::new(from_file(json_file)).fetch()?.as_json();

        let resp = cfg.unwrap().unwrap();
        assert_eq!(resp.clone().message, "hello world");
        assert_eq!(resp.clone().keys.key1, "value1");
        assert_eq!(resp.clone().keys.key2, "value2");
        Ok(())
    }
}
