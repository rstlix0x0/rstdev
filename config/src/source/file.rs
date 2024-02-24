use std::fs;

use crate::types::{ConfigError, SourceParser};
use crate::{Format, StringValue};

/// File is an adapter that will fetch the configuration as a string
/// from a given input file path
pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl SourceParser<StringValue> for File {
    fn fetch(&self) -> Result<Format<StringValue>, ConfigError> {
        let content = fs::read_to_string(self.filepath.clone())
            .map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Ok(Format::new(StringValue::new(content)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rst_common::standard::serde::{self, Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "self::serde")]
    struct Message {
        message: String,
    }

    #[test]
    fn test_parse_yaml_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("fixtures");

        let yaml_file = format!("{}/test.yaml", path.display());
        let source_file = File::new(yaml_file);
        let output = source_file.fetch();
        assert!(!output.is_err());

        let yaml: Option<Result<Message, ConfigError>> = output.unwrap().as_yaml();
        assert!(yaml.is_some());
        assert_eq!("hello world".to_string(), yaml.unwrap().unwrap().message)
    }
}
