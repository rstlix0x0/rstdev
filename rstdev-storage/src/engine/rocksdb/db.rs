//! This module provide a main object of [`DB`] which is an object that
//! will maintain `RocksDB` object's instance
use std::sync::Arc;

use rust_rocksdb::ColumnFamilyDescriptor;
use rust_rocksdb::DB as CoreDB;

use crate::types::Storage;

use super::options::Options;
use super::types::RocksDBError;

/// `DB` is a main object that depend on [`Options`] and used to build
/// `RocksDB` database instance
#[derive(Clone)]
pub struct DB {
    pub(crate) db: Option<Arc<CoreDB>>,
    opts: Options,
}

impl DB {
    pub fn new(opts: Options) -> Result<Self, RocksDBError> {
        let _ = opts
            .validate()
            .map_err(|err| RocksDBError::InstanceError(err.to_string()))?;

        Ok(Self { opts, db: None })
    }

    pub fn set_db(&mut self, instance: Option<Arc<CoreDB>>) -> &mut Self {
        self.db = instance;
        self
    }

    pub fn build(&self) -> Result<Option<Arc<CoreDB>>, RocksDBError> {
        // database instance already been setup it should not re-create
        // the instance anymore
        if self.db.is_some() {
            return Ok(self.db.clone());
        }

        let db_path = self.opts.path.clone();
        let cf_name = self.opts.cf_name.clone();

        let cf_opts =
            self.opts
                .cf_opts
                .as_ref()
                .map(|val| val)
                .ok_or(RocksDBError::InstanceError(
                    "cf options was empty".to_string(),
                ))?;

        let cf_descriptor = ColumnFamilyDescriptor::new(cf_name, cf_opts.clone());

        let db_opts =
            self.opts
                .db_opts
                .as_ref()
                .map(|val| val)
                .ok_or(RocksDBError::InstanceError(
                    "db options was empty".to_string(),
                ))?;

        let db = CoreDB::open_cf_descriptors(db_opts, db_path, vec![cf_descriptor])
            .map_err(|err| RocksDBError::InstanceError(err.to_string()))?;

        let db_instance = Some(Arc::new(db));
        Ok(db_instance)
    }
}

impl Storage for DB {
    type Instance = Self;

    fn get_instance(self) -> Self::Instance {
        self
    }
}
