use crate::BaseError;

/// `ToJSON` used when an entity want to convert themself into
/// json encoding format
pub trait ToJSON {
    fn to_json(&self) -> Result<String, BaseError>;
}

/// `Validate` should be used when an entity need to validate their properties
pub trait Validate {
    fn validate(&self) -> Result<(), BaseError>;
}

/// `UID` is a value object used as unique identity that must have by each of identity
///
/// This trait using associated type of [`UID::Value`] to fill any possible types. Usually
/// it will either a string (`UUID`) or an integer (`AUTO_INCREMENT`), but it also be able
/// to used an id like `MongoDB ID Object Hash`
pub trait UID {
    type Value;

    fn uid(&self) -> Self::Value;
}