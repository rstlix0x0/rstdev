use std::marker::PhantomData;

use crate::types::{ConfigError, SourceFormatter, SourceParser};
use crate::Source;

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
pub struct Builder<TParser, TFormatter, TValue>
where
    TValue: Clone,
    TFormatter: for<'a> SourceFormatter<'a, TValue>,
    TParser: for<'a> SourceParser<TFormatter, TValue>,
{
    adapter: TParser,
    _phantomf: Option<PhantomData<TFormatter>>,
    _phantomf2: Option<PhantomData<TValue>>,
}

impl<T, F, St> Builder<T, F, St>
where
    St: Clone,
    F: for<'a> SourceFormatter<'a, St>,
    T: for<'a> SourceParser<F, St>,
{
    pub fn new(adapter: T) -> Self {
        Self {
            adapter,
            _phantomf: None,
            _phantomf2: None,
        }
    }

    pub fn fetch(&self) -> Result<Source<F, St>, ConfigError> {
        self.adapter.fetch()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use rst_common::standard::serde::{self, Deserialize, Serialize};

    use crate::format::{from_toml, from_yaml, from_json};
    use crate::parser::from_file;

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
    fn test_parser_file_toml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let toml_file = format!("{}/test.toml", path.display());
        let cfg: MessageGroup = Builder::new(from_file(toml_file))
            .fetch()?
            .parse(from_toml)?;

        assert_eq!(cfg.clone().message, "hello world");
        assert_eq!(cfg.clone().keys.key1, "value1");
        assert_eq!(cfg.clone().keys.key2, "value2");
        Ok(())
    }

    #[test]
    fn test_parser_file_yaml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let yaml_file = format!("{}/test.yaml", path.display());
        let cfg: Message = Builder::new(from_file(yaml_file))
            .fetch()?
            .parse(from_yaml)?;

        assert_eq!(cfg.clone().message, "hello world");
        Ok(())
    }

    #[test]
    fn test_parser_file_json() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");
        
        let json_file = format!("{}/test.json", path.display());
        let cfg: MessageGroup = Builder::new(from_file(json_file))
            .fetch()?
            .parse(from_json)?;

        assert_eq!(cfg.clone().message, "hello world");
        assert_eq!(cfg.clone().keys.key1, "value1");
        assert_eq!(cfg.clone().keys.key2, "value2");
        Ok(())
    }
}
