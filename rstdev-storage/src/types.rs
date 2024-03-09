//! `types` module provides base data types structure which consists of [`Storage`] and 
//! [`StorageError`]

use rst_common::standard::async_trait::async_trait;
use rst_common::with_errors::thiserror::{self, Error};

/// `StorageError` is a list of error types designed specifically
/// for common storage activities
#[derive(Debug, Error, PartialEq)]
pub enum StorageError {
    #[error("storage connection error: {0}")]
    ConnectionError(String),

    #[error("storage ping error: {0}")]
    PingError(String),

    #[error("storage query error: {0}")]
    QueryError(String),

    #[error("storage trait not implemented: {0}")]
    MethodNotImplementedError(String),

    #[error("options error: {0}")]
    BuildOptionsError(String),
}

/// `Storage` trait is a core abstraction for external storage
///
/// This trait should be implemented by any external storage implementation
/// like MySQL or PostgreSQL or even Redis.
///
/// This trait should not give too many constraints to any implementors but still
/// provides basic common methods. There is only a single required method that need
/// to be implemented which is [`Storage::get_instance`], other methods are optional.
/// For all optional methods, you must override the implementation, by default it will
/// throw a [`StorageError::MethodNotImplementedError`]
///
/// This trait designed to working side by side with the `Repository Pattern` as persistent
/// layer to the external storage
#[async_trait]
pub trait Storage {
    /// `Instance` is an associated type that represent an storage object instance
    /// This type has a constraint that any object that become this type, must be also
    /// implement `Send` and `Sync` to make it sure for thread-safe operations
    type Instance: Send + Sync;

    /// `get_instance` is a base method used to get `Instance` object, whatever it is
    fn get_instance(self) -> Self::Instance;

    /// `connect` is an optional method to connecting to some external storage
    ///
    /// This method designed to be optional, to make it flexible for implementor
    /// to decide they need this or not. Maybe not all available libraries provide
    /// this mechanism
    async fn connect(&self) -> Result<(), StorageError> {
        Err(StorageError::MethodNotImplementedError(
            "connect".to_string(),
        ))
    }

    /// `ping` is an optional method used to try to ping our external storage
    ///
    /// This method will be useful if we have a case where we need to make sure
    /// that our current connection still healthy
    async fn ping(&mut self) -> Result<(), StorageError> {
        Err(StorageError::MethodNotImplementedError("ping".to_string()))
    }

    /// `close` is an optional method used to close storage connection
    async fn close(&mut self) -> Result<(), StorageError> {
        Err(StorageError::MethodNotImplementedError("close".to_string()))
    }
}
