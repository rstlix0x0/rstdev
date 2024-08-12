use rst_common::with_errors::thiserror::{self, Error};

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

/// `ToJSON` used when an entity want to convert themself into
/// json encoding format
pub trait ToJSON {
    fn to_json(&self) -> Result<String, BaseError>;
}

/// `Validate` should be used when an entity need to validate their properties
pub trait Validate {
    fn validate(&self) -> Result<(), BaseError>;
}