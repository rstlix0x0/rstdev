use rst_common::with_errors::thiserror::{self, Error};

#[derive(Debug, Error, PartialEq)]
pub enum RocksDBError {
    #[error("validate error: {0}")]
    ValidateError(String),

    #[error("instance error: {0}")]
    InstanceError(String),
}
