//! `options` used to manage all necessary options to setup Postgres database instance
//! and connection
use sqlx::pool::PoolOptions;
use sqlx::postgres::{PgConnectOptions, Postgres};

use crate::types::StorageError;

use crate::sql::options::{DefaultDBOptions, DefaultDBPoolOptions, DefaultDBPoolOptionsBuilder};
use crate::sql::types::{SqlxOptionsBuilder, SqlxPoolOptionsBuilder};

const DEFAULT_PORT: u16 = 5432;

/// `Options` will hold two kind of options, a main database options and pooled options
/// 
/// This object also implement [`SqlxOptionsBuilder`] to build common database options
/// and also [`SqlxPoolOptionsBuilder`] to build pooled options.
/// 
/// For the pooled options, it will be used [`DefaultDBPoolOptionsBuilder`]
pub struct Options {
    app_name: Option<String>,
    db_opts: DefaultDBOptions,
    pool_opts: DefaultDBPoolOptions,
}

impl Options {
    pub fn new(
        db_opts: DefaultDBOptions,
        pool_opts: DefaultDBPoolOptions,
        app_name: Option<String>,
    ) -> Result<Self, StorageError> {
        let _ = db_opts.validate()?;
        Ok(Self {
            db_opts,
            pool_opts,
            app_name,
        })
    }
}

impl SqlxOptionsBuilder for Options {
    type SqlxOptionType = PgConnectOptions;

    fn common_options(&self) -> Self::SqlxOptionType {
        let mut pg_opts = PgConnectOptions::new()
            .username(&self.db_opts.username.as_str())
            .database(&self.db_opts.db.as_str())
            .password(&self.db_opts.password.as_str());

        if let Some(host) = &self.db_opts.host {
            pg_opts = pg_opts.host(host.to_owned().as_str());
        }

        if let Some(app_name) = &self.app_name {
            pg_opts = pg_opts.application_name(app_name.to_owned().as_str());
        }

        match self.db_opts.port {
            Some(port) => pg_opts.port(port),
            None => pg_opts.port(DEFAULT_PORT),
        }
    }
}

impl SqlxPoolOptionsBuilder for Options {
    type SqlxDatabase = Postgres;

    fn pool_options(&self) -> PoolOptions<Self::SqlxDatabase> {
        DefaultDBPoolOptionsBuilder::<Self::SqlxDatabase>::new(self.pool_opts.to_owned()).build()
    }
}
