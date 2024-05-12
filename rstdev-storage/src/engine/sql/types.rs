//! `types` module provide basic base trait behaviors to build `sqlx` object dependencies
//!
//! Since all `sql` objects built on top of `sqlx`, all types defined here designed as a bridge
//! abstraction between the `sqlx` and our `Storage` needs
//!
//! There are three important concepts that need to be abstracted:
//! - [`SqlxOptionsBuilder`] used as trait behavior to build options for a single database connection
//! - [`SqlxPoolOptionsBuilder`] used as trait behavior to build pool options for a pool database connection
//! - [`SqlxConnectionBuilder`] used as a trait behavior to build database connection

use sqlx::pool::{Pool, PoolOptions};
use sqlx::Connection;
use sqlx::Database;

use rst_common::standard::async_trait::async_trait;

use crate::types::StorageError;

/// `SqlxOptionsBuilder` used to build an option for a single connection
///
/// Actually this options will be used too at pool options that these option properties
/// used as base database options such as, host, port, username, password and database
pub trait SqlxOptionsBuilder {
    type SqlxOptionType;

    fn common_options(&self) -> Self::SqlxOptionType;
}

/// `SqlxPoolOptionsBuilder` used to build an option for a pool database connection
///
/// A pool database connection build to maintain multiple database connection, and in `sqlx`
/// it's a separated options. This trait need to define it's associated type, [`SqlxPoolOptionsBuilder::SqlxDatabase`]
/// which is an object that implement sqlx [`Database`]
pub trait SqlxPoolOptionsBuilder {
    type SqlxDatabase: Database;

    fn pool_options(&self) -> PoolOptions<Self::SqlxDatabase>;
}

/// `SqlxConnectionBuilder` used to build database connections which consists of two type of connection
/// a single and pooled connections
///
/// In `sqlx`, the connection and the database instance are separated object. This trait also provide two
/// required methods to build a single connection and pooled connection
#[async_trait]
pub trait SqlxConnectionBuilder {
    type SqlxDbConnection: Connection;
    type SqlxDb: Database;

    async fn build_single_conn(&self) -> Option<Result<Self::SqlxDbConnection, StorageError>>;
    async fn build_pool_conn(&self) -> Option<Result<Pool<Self::SqlxDb>, StorageError>>;
}
