//! `db` is a module that provide an implementation of [`Storage`] and also [`SqlxConnectionBuilder`]
use std::sync::Arc;

use rst_common::standard::async_trait::async_trait;
use sqlx::mysql::{MySql, MySqlConnection};
use sqlx::pool::Pool;
use sqlx::Connection;

use crate::engine::sql::mysql::options::Options;
use crate::engine::sql::types::{SqlxConnectionBuilder, SqlxOptionsBuilder, SqlxPoolOptionsBuilder};
use crate::types::{Storage, StorageError};

/// `DB` will depends to [`Options`] to setup it's database connection and also it's instance
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
    type SqlxDb = MySql;
    type SqlxDbConnection = MySqlConnection;

    async fn build_single_conn(&self) -> Option<Result<Self::SqlxDbConnection, StorageError>> {
        let mysql_opts = self.opts.common_options();
        let single_conn = MySqlConnection::connect_with(&mysql_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()));

        Some(single_conn)
    }

    async fn build_pool_conn(&self) -> Option<Result<Pool<Self::SqlxDb>, StorageError>> {
        let mysql_opts = self.opts.common_options();
        let pool_conn_opts = self.opts.pool_options();
        let pool_conn = pool_conn_opts
            .connect_with(mysql_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()));

        Some(pool_conn)
    }
}

/// `MysqlDB` is an object that implement [`Storage`], this object will depends to [`DB`] to build
/// it's connection types, a common or pooled connections
pub struct MysqlDB {
    db: DB,
    pub single_conn: Option<MySqlConnection>,
    pub pool_conn: Option<Pool<MySql>>,
}

impl MysqlDB {
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
impl Storage for MysqlDB {
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
