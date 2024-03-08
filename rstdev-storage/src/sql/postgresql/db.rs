use std::sync::Arc;

use rst_common::standard::async_trait::async_trait;
use sqlx::pool::Pool;
use sqlx::postgres::{PgConnection, Postgres};
use sqlx::Connection;

use crate::sql::postgresql::options::Options;
use crate::sql::types::{SqlxConnectionBuilder, SqlxOptionsBuilder, SqlxPoolOptionsBuilder};
use crate::types::{Storage, StorageError};

pub struct DB {
    opts: Options,
}

impl DB {
    pub fn new(opts: Options) -> Self {
        Self { opts }
    }
}

#[async_trait]
impl SqlxConnectionBuilder for DB {
    type SqlxDb = Postgres;
    type SqlxDbConnection = PgConnection;

    async fn build_single_conn(&self) -> Option<Result<Self::SqlxDbConnection, StorageError>> {
        let pg_opts = self.opts.common_options();
        let single_conn = PgConnection::connect_with(&pg_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()));

        Some(single_conn)
    }

    async fn build_pool_conn(&self) -> Option<Result<Pool<Self::SqlxDb>, StorageError>> {
        let pg_opts = self.opts.common_options();
        let pool_conn_opts = self.opts.pool_options();
        let pool_conn = pool_conn_opts
            .connect_with(pg_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()));

        Some(pool_conn)
    }
}

pub struct PostgresqlDB {
    db: DB,
    pub single_conn: Option<PgConnection>,
    pub pool_conn: Option<Pool<Postgres>>,
}

impl PostgresqlDB {
    pub fn new(db: DB) -> Self {
        Self {
            db,
            single_conn: None,
            pool_conn: None,
        }
    }

    pub async fn build_conns(&mut self) -> Result<(), StorageError> {
        let single_conn = self.db.build_single_conn().await;
        let pool_conn = self.db.build_pool_conn().await;

        if single_conn.is_some() {
            let single_conn_result = single_conn.unwrap()?;
            self.single_conn = Some(single_conn_result);
        }

        if pool_conn.is_some() {
            let pool_conn_result = pool_conn.unwrap()?;
            self.pool_conn = Some(pool_conn_result)
        }

        Ok(())
    }
}

#[async_trait]
impl Storage for PostgresqlDB {
    type Instance = Arc<Self>;

    fn get_instance(self) -> Self::Instance {
        Arc::new(self)
    }

    async fn ping(&mut self) -> Result<(), StorageError> {
        self.single_conn
            .as_mut()
            .map(|val| async move {
                val.ping()
                    .await
                    .map_err(|err| StorageError::PingError(err.to_string()))
            })
            .ok_or(StorageError::PingError("unable to ping".to_string()))?
            .await
    }

    async fn close(&mut self) -> Result<(), StorageError> {
        if self.single_conn.is_some() {
            self.single_conn
                .take()
                .map(|val| async move {
                    val.close()
                        .await
                        .map_err(|err| StorageError::ConnectionError(err.to_string()))
                })
                .ok_or(StorageError::ConnectionError(
                    "unable to close the connection".to_string(),
                ))?
                .await?;
        }

        if self.pool_conn.is_some() {
            self.pool_conn
                .take()
                .map(|val| async move { val.close().await })
                .ok_or(StorageError::ConnectionError(
                    "unable to close the pool connection".to_string(),
                ))?
                .await;
        }

        Ok(())
    }
}
