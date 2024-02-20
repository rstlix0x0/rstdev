mod format;
mod types;

pub mod source;

pub use format::Format;
pub use types::{ConfigError, SourceParser};

pub struct Builder<T>
where
    T: SourceParser,
{
    adapter: T,
}

impl<T> Builder<T>
where
    T: SourceParser,
{
    pub fn new(adapter: T) -> Self {
        Self { adapter }
    }

    pub fn fetch(&self) -> Result<Format, ConfigError> {
        self.adapter.fetch()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use source::from_file;
    use rst_common::standard::serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug)]
    struct Message {
        message: String,
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct MessageGroup {
        message: String,
        keys: MessageGroupKeys
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct MessageGroupKeys {
        key1: String,
        key2: String
    }

    #[test]
    fn test_parse_config_yaml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let yaml_file = format!("{}/test.yaml", path.display());
        let cfg: Message = Builder::new(from_file(yaml_file))
            .fetch()?
            .as_yaml()?;

        assert_eq!(cfg.message, "hello world");
        Ok(())
    }

    #[test]
    fn test_parse_config_toml() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let toml_file = format!("{}/test.toml", path.display());
        let cfg: MessageGroup = Builder::new(from_file(toml_file))
            .fetch()?
            .as_toml()?;

        assert_eq!(cfg.message, "hello world");
        assert_eq!(cfg.keys.key1, "value1");
        assert_eq!(cfg.keys.key2, "value2");
        Ok(())
    }

    #[test]
    fn test_parse_config_json() -> Result<(), ConfigError> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let json_file = format!("{}/test.json", path.display());
        let cfg: MessageGroup = Builder::new(from_file(json_file))
            .fetch()?
            .as_json()?;

        assert_eq!(cfg.message, "hello world");
        assert_eq!(cfg.keys.key1, "value1");
        assert_eq!(cfg.keys.key2, "value2");
        Ok(())
    }
}
