//! This module used to maintain all `RocksDB` database options including for
//! required `path` and `cf_name`. This options used with an assumption that
//! the caller will use `ColumnFamily` feature from `RocksDB`
use rust_rocksdb::Options as CoreOptions;

use super::types::RocksDBError;

/// `Options` used to provides field properties. The required properties are:
/// - `path`
/// - `cf_name`
///
/// Actually the `db_opts` and `cf_opts` is required too, but when this object
/// build it will initialize with empty object (`None`), later the caller still need to
/// to build the object
#[derive(Clone)]
pub struct Options {
    pub(crate) path: String,
    pub(crate) cf_name: String,
    pub(crate) db_opts: Option<CoreOptions>,
    pub(crate) cf_opts: Option<CoreOptions>,
}

impl Options {
    pub fn new(path: String, cf_name: String) -> Self {
        Self {
            path,
            cf_name,
            cf_opts: None,
            db_opts: None,
        }
    }

    pub fn build_default_opts(&mut self) -> &mut Self {
        self.db_opts = Some(CoreOptions::default());
        self.cf_opts = Some(CoreOptions::default());
        self
    }

    pub fn set_db_opts(
        &mut self,
        mut callback: impl FnMut(&mut CoreOptions) -> &mut CoreOptions,
    ) -> &mut Self {
        let db_opts = self.db_opts.as_mut().map(move |val| callback(val));
        self.db_opts = db_opts.cloned();
        self
    }

    pub fn set_cf_opts(
        &mut self,
        mut callback: impl FnMut(&mut CoreOptions) -> &mut CoreOptions,
    ) -> &mut Self {
        let cf_opts = self.cf_opts.as_mut().map(move |val| callback(val));
        self.cf_opts = cf_opts.cloned();
        self
    }

    pub fn validate(&self) -> Result<(), RocksDBError> {
        if self.path.is_empty() {
            return Err(RocksDBError::ValidateError("db path is empty".to_string()));
        }

        if self.cf_name.is_empty() {
            return Err(RocksDBError::ValidateError(
                "column family name is empty".to_string(),
            ));
        }

        if self.cf_opts.is_none() {
            return Err(RocksDBError::ValidateError(
                "missing cf options".to_string(),
            ));
        }

        if self.db_opts.is_none() {
            return Err(RocksDBError::ValidateError(
                "missing db options".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_success() {
        let opts = Options::new("./db".to_string(), "cf-name".to_string())
            .build_default_opts()
            .validate();

        assert!(!opts.is_err())
    }

    #[test]
    fn test_validation_error_empty_path() {
        let opts = Options::new("".to_string(), "cf-name".to_string())
            .build_default_opts()
            .validate();

        assert!(opts.is_err());
        assert!(opts.unwrap_err().to_string().contains("db path is empty"))
    }

    #[test]
    fn test_validation_error_empty_cf_name() {
        let opts = Options::new("./db".to_string(), "".to_string())
            .build_default_opts()
            .validate();

        assert!(opts.is_err());
        assert!(opts
            .unwrap_err()
            .to_string()
            .contains("column family name is empty"))
    }

    #[test]
    fn test_validation_error_empty_db_opts() {
        let mut opts = Options::new("./db".to_string(), "cf-name".to_string());
        opts.build_default_opts();
        opts.db_opts = None;

        let validation = opts.validate();
        assert!(validation.is_err());
        assert!(validation
            .unwrap_err()
            .to_string()
            .contains("missing db options"))
    }

    #[test]
    fn test_validation_error_empty_cf_opts() {
        let mut opts = Options::new("./db".to_string(), "cf-name".to_string());
        opts.build_default_opts();
        opts.cf_opts = None;

        let validation = opts.validate();
        assert!(validation.is_err());
        assert!(validation
            .unwrap_err()
            .to_string()
            .contains("missing cf options"))
    }

    #[test]
    fn test_set_db_opts_none() {
        let mut opts = Options::new("./db".to_string(), "cf-name".to_string());
        opts.set_db_opts(|val| val);

        assert!(opts.db_opts.is_none())
    }

    #[test]
    fn test_set_cf_opts_none() {
        let mut opts = Options::new("./db".to_string(), "cf-name".to_string());
        opts.set_cf_opts(|val| val);

        assert!(opts.cf_opts.is_none())
    }
}
