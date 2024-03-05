use core::time::Duration;

use crate::types::StorageError;

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

#[derive(Debug, Clone)]
pub struct DefaultDBPoolOptions {
    pub max_conns: Option<u32>,
    pub min_conns: Option<u32>,
    pub idle_duration: Option<Duration>,
    pub lifetime_duration: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
}
