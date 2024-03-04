use std::sync::Arc;

use sqlx::mysql::{MySql, MySqlConnection};
use sqlx::pool::Pool;
use sqlx::Connection;

use rst_common::standard::async_trait::async_trait;

use crate::mysql::{MySQLError, MySQLOptions, MySQLPoolOptions};
use crate::types::{Storage, StorageError};

pub struct DB {
    db_opts: MySQLOptions,
    single_conn: Option<MySqlConnection>,
    pool_conn: Option<Pool<MySql>>,
}

impl DB {
    pub fn new(db_opts: MySQLOptions) -> Result<Self, MySQLError> {
        let _ = db_opts.validate()?;
        Ok(Self {
            db_opts,
            single_conn: None,
            pool_conn: None,
        })
    }

    pub async fn build_single_conn(&mut self) -> Result<(), StorageError> {
        let mysql_opts = self.db_opts.to_sqlx_options();
        let single_conn = MySqlConnection::connect_with(&mysql_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()))?;

        self.single_conn = Some(single_conn);
        Ok(())
    }

    pub fn build_pool_opts(&self) -> Result<MySQLPoolOptions, StorageError> {
        MySQLPoolOptions::new(self.db_opts.clone())
            .map_err(|err| StorageError::ConnectionError(err.to_string()))
    }

    pub async fn build_pool_conn(
        &mut self,
        pool_opts: MySQLPoolOptions,
    ) -> Result<(), StorageError> {
        let mysql_opts = self.db_opts.to_sqlx_options();
        let pool_conn_opts = pool_opts.to_sqlx_pool_options();
        let pool_conn = pool_conn_opts
            .connect_with(mysql_opts)
            .await
            .map_err(|err| StorageError::ConnectionError(err.to_string()))?;

        self.pool_conn = Some(pool_conn);
        Ok(())
    }
}

pub struct MySQLDB {
    db: DB,
}

impl MySQLDB {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Storage for MySQLDB {
    type Instance = Arc<DB>;

    fn get_instance(self) -> Self::Instance {
        Arc::new(self.db)
    }

    async fn ping(&mut self) -> Result<(), StorageError> {
        self.db
            .single_conn
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
        if self.db.single_conn.is_some() {
            self.db
                .single_conn
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

        if self.db.pool_conn.is_some() {
            self.db
                .pool_conn
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
