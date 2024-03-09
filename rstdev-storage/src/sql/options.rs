//! `options` provides an implementation basic data structure to hold database main and pooled
//! options
use core::time::Duration;
use std::marker::PhantomData;

use sqlx::pool::PoolOptions;
use sqlx::Database;

use crate::types::StorageError;

/// `DefaultDBOptions` used as main and primary database options
/// 
/// This object struct also provide associated method to [`DefaultDBOptions::validate`] current
/// given option fields
#[derive(Debug, Clone)]
pub struct DefaultDBOptions {
    pub host: Option<String>,
    pub username: String,
    pub password: String,
    pub db: String,
    pub port: Option<u16>,
}

impl DefaultDBOptions {
    pub fn validate(&self) -> Result<(), StorageError> {
        if self.username.is_empty() {
            return Err(StorageError::BuildOptionsError(
                "username is empty".to_string(),
            ));
        }

        if self.password.is_empty() {
            return Err(StorageError::BuildOptionsError(
                "password is empty".to_string(),
            ));
        }

        if self.db.is_empty() {
            return Err(StorageError::BuildOptionsError("db is empty".to_string()));
        }

        Ok(())
    }
}

/// `DefaultDBPoolOptions` used to hold database pool options
/// 
/// Not all `sqlx` pool options defined here, I only set a necessary properties, and all of them
/// are optional. Sqlx already give a default value for each option fields unless we also
/// set them
#[derive(Debug, Clone)]
pub struct DefaultDBPoolOptions {
    pub max_conns: Option<u32>,
    pub min_conns: Option<u32>,
    pub idle_duration: Option<Duration>,
    pub lifetime_duration: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
}

/// `DefaultDBPoolOptionsBuilder` this object used as default implementation of sqlx pool option builder
/// 
/// The motivatino of this object is used as shared object that will be able to be used between current
/// Mysql and Postgres, rather than implement it manually to each of database instance
pub struct DefaultDBPoolOptionsBuilder<TDB>
where
    TDB: Database,
{
    opts: DefaultDBPoolOptions,
    _marker: Option<PhantomData<TDB>>,
}

impl<TDB> DefaultDBPoolOptionsBuilder<TDB>
where
    TDB: Database,
{
    pub fn new(opts: DefaultDBPoolOptions) -> Self {
        Self {
            opts,
            _marker: None,
        }
    }

    pub fn build(&self) -> PoolOptions<TDB> {
        let mut pool_opts = PoolOptions::<TDB>::new();
        if let Some(max_conns) = &self.opts.max_conns {
            pool_opts = pool_opts.clone().max_connections(max_conns.to_owned());
        }

        if let Some(min_conns) = &self.opts.min_conns {
            pool_opts = pool_opts.clone().min_connections(min_conns.to_owned());
        }

        if let Some(idle) = &self.opts.idle_duration {
            pool_opts = pool_opts.clone().idle_timeout(idle.to_owned());
        }

        if let Some(lifetime) = &self.opts.lifetime_duration {
            pool_opts = pool_opts.clone().max_lifetime(lifetime.to_owned());
        }

        if let Some(acquire) = &self.opts.acquire_timeout {
            pool_opts = pool_opts.clone().acquire_timeout(acquire.to_owned());
        }

        pool_opts
    }
}
