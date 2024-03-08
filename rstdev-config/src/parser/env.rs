use std::env;

use crate::types::{ConfigError, SourceParser};
use crate::values::TupleValue;
use crate::Source;

/// `Env` is an adapter that built to read all available
/// environment variables based on specific `PREFIX_`
///
/// The `PREFIX_` is a required parameter. This decision made
/// because there are will have so many possibilites of env vars,
/// so we should separate which vars belongs to our system which not.
///
/// The `PREFIX_` also used to prevent conflicted variable names between
/// our system needs and with the external system, since we cannot control
/// external variable names outside of our system.
pub struct Env {
    prefix: String,
}

impl Env {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

impl SourceParser<TupleValue, Vec<(String, String)>> for Env {
    fn fetch(&self) -> Result<Source<TupleValue, Vec<(String, String)>>, ConfigError> {
        let vars = env::vars();
        let input_vars: Vec<(String, String)> = vars.into_iter().collect();
        let input_source: Vec<(String, String)> = input_vars
            .into_iter()
            .filter(|(key, _)| key.starts_with(&self.prefix))
            .map(|(key, value)| (key.trim_start_matches(&self.prefix).to_owned(), value))
            .collect();

        if input_source.len() < 1 {
            return Err(ConfigError::FormatError(
                "no environment variables available".to_string(),
            ));
        }

        Ok(Source::new(TupleValue::new(input_source)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, env};

    #[test]
    fn test_fetch() {
        env::set_var("TEST_KEY1", "value1");
        env::set_var("TEST_KEY2", "value2");

        let parser = Env::new("TEST_".to_string());
        let source = parser.fetch();
        assert!(!source.is_err());

        let result = source.unwrap().parse(|input| {
            let mut mapper: HashMap<String, String> = HashMap::new();
            for (key, value) in input.into_iter() {
                if key == "KEY1" {
                    mapper.insert(key.clone(), value.clone());
                }

                if key == "KEY2" {
                    mapper.insert(key.clone(), value.clone());
                }
            }

            if mapper.len() < 1 {
                return Err(ConfigError::FormatError(
                    "no env vars available".to_string(),
                ));
            }

            Ok(mapper)
        });
        assert!(!result.is_err());

        let mapper = result.unwrap();
        assert_eq!(mapper.get("KEY1").unwrap().to_owned(), "value1".to_string());
        assert_eq!(mapper.get("KEY2").unwrap().to_owned(), "value2".to_string())
    }

    #[test]
    fn test_fetch_no_vars() {
        env::set_var("KEY1", "value1");
        env::set_var("KEY2", "value2");

        let parser = Env::new("INVALID_".to_string());
        let source = parser.fetch();
        assert!(source.is_err());
        assert!(matches!(
            source.as_ref().unwrap_err(),
            ConfigError::FormatError(_)
        ));
        assert_eq!(
            source.unwrap_err(),
            ConfigError::FormatError("no environment variables available".to_string())
        )
    }
}
