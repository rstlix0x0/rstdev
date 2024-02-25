use std::fs;

use crate::types::{ConfigError, SourceParser};
use crate::Source;
use crate::values::StringValue;

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

impl SourceParser<StringValue, String> for File {
    fn fetch(&self) -> Result<Source<StringValue, String>, ConfigError> {
        let content = fs::read_to_string(self.filepath.clone())
            .map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Ok(Source::new(StringValue::new(content)))
    }
}
