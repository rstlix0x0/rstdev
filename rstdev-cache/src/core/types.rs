use rst_common::standard::async_trait::async_trait;
use rst_common::with_errors::thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("storage error: {0}")]
    StorageError(String),

    #[error("invalidate error: {0}")]
    InvalidateError(String),

    #[error("eviction error: {0}")]
    EvictionError(String),
}

/// `ValueBuilder` used for `key` and `value` type management
///
/// This trait must be implemented by any key and value that willing to be used
/// in storage registry. This trait designed to give engineers flexibility to
/// manage their key and value management, which means it can be anything depends
/// on their domain's neeeds, rather than force engineers to using primitive types
/// like `String` or `u32` or even bytes.
///
/// If a value implement this trait, an engineer will be able to do anything, maybe like
/// compress their `key` or `value`
pub trait ValueBuilder {
    type Value;

    fn value(&self) -> Self::Value;
}

/// `StorageBuilder` used as base abstraction to manage read and write operation to
/// any cache storage implementer, maybe something like `Redis`
///
/// The data saved in cache storage is a key-value data types that their types must be implement
/// trait of [`ValueBuilder`]
#[async_trait]
pub trait StorageBuilder {
    async fn set<TKey, TValue>(
        &self,
        key: impl ValueBuilder<Value = TKey>,
        val: impl ValueBuilder<Value = TValue>,
    );

    async fn get<TKey, TValue>(
        &self,
        key: impl ValueBuilder<Value = TKey>,
    ) -> Result<Option<Box<dyn ValueBuilder<Value = TValue>>>, CoreError>;

    async fn remove<TKey>(&self, key: impl ValueBuilder<Value = TKey>) -> Result<(), CoreError>;
}

/// `InvalidatePolicyBuilder` used as base abstraction for invalidation policy
///
/// Invalidation used in cache mechanism to set a saved `key` to be expired. For any implementers
/// that want to implement this trait, it may be has a relation with [`StorageBuilder`]
#[async_trait]
pub trait InvalidatePolicyBuilder {
    async fn invalidate<TKey>(&self, key: impl ValueBuilder<Value = TKey>)
        -> Result<(), CoreError>;
}

/// `EvictionPolicyBuilder` used as base abstraction for eviction policy
///
/// Eviction used in cache mechanism to remove unused `key` to give some free space, this policy mechanism
/// will be used to manage cache key's capacities
#[async_trait]
pub trait EvictionPolicyBuilder {
    async fn evict<TKey>(&self, key: impl ValueBuilder<Value = TKey>) -> Result<(), CoreError>;
}

/// `PolicyBuilder` used as base abstraction to build policy management which should contains of
/// [`InvalidatePolicyBuilder`] and [`EvictionPolicyBuilder`]
pub trait PolicyBuilder {
    fn set_invalidation_policy(&mut self, invalidate: impl InvalidatePolicyBuilder);
    fn set_eviction_policy(&mut self, eviction: impl EvictionPolicyBuilder);
}

/// `LayerBuilder` is the highest base abstraction which will contains [`StorageBuilder`]
/// and [`PolicyBuilder`]
pub trait LayerBuilder {
    fn set_storage(&mut self, storage_impl: impl StorageBuilder);
    fn policy<TPolicy: PolicyBuilder>(&self) -> TPolicy;
}
