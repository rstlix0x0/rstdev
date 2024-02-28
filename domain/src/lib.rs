#![doc = include_str!("../README.md")]

use rst_common::with_errors::thiserror::{self, Error};

pub mod aggregate;
pub mod entity;
pub mod repository;

/// `EntityBaseError` provides basic common error that probably will be used
/// by any type of domain entities
#[derive(Debug, Error)]
pub enum BaseError {
    #[error("unable to convert to json: {0}")]
    ToJSONError(String),

    #[error("validation failed: {0}")]
    ValidateError(String),
    
    #[error("unable to publish an event: {0}")]
    PublishError(String),

    #[error("unable to emit event: {0}")]
    EmitError(String),

    #[error("unable to handle an event: {event_name}, error: {error_msg}")]
    HandleError {
        event_name: String,
        error_msg: String,
    },

    #[error("repository error: {0}")]
    RepositoryError(String)
}