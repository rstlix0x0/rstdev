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
}
