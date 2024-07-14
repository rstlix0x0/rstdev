//! This module only provide a very simple enum of [`RocksDBError`] that only contains
//! two enum's keys
use rst_common::with_errors::thiserror::{self, Error};

#[derive(Debug, Error, PartialEq)]
pub enum RocksDBError {
    #[error("validate error: {0}")]
    ValidateError(String),

    #[error("instance error: {0}")]
    InstanceError(String),

    #[error("executor error: {0}")]
    ExecutorError(String),
}

pub enum Instruction {
    SaveCf { key: String, value: Vec<u8> },
    MergeCf { key: String, value: Vec<u8> },
    GetCf { key: String },
    MultiGetCf { keys: Vec<String> },
    RemoveCf { key: String },
}

#[derive(Debug)]
pub enum OutputOpts {
    SingleByte {
        value: Option<Vec<u8>>,
    },
    MultiBytes {
        values: Vec<Result<Option<Vec<u8>>, RocksDBError>>,
    },
    None,
}
