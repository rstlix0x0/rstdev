use sqlx::pool::{Pool, PoolOptions};
use sqlx::Connection;
use sqlx::Database;

use rst_common::standard::async_trait::async_trait;

use crate::types::StorageError;

pub trait SqlxOptionsBuilder {
    type SqlxOptionType;

    fn common_options(&self) -> Self::SqlxOptionType;
}

pub trait SqlxPoolOptionsBuilder {
    type SqlxDatabase: Database;

    fn pool_options(&self) -> PoolOptions<Self::SqlxDatabase>;
}

#[async_trait]
pub trait SqlxConnectionBuilder {
    type SqlxDbConnection: Connection;
    type SqlxDb: Database;

    async fn build_single_conn(&self) -> Option<Result<Self::SqlxDbConnection, StorageError>>;
    async fn build_pool_conn(&self) -> Option<Result<Pool<Self::SqlxDb>, StorageError>>;
}
