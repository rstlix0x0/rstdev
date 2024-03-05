use sqlx::postgres::{Postgres, PgConnectOptions};
use sqlx::pool::PoolOptions;

use crate::types::StorageError;

use crate::sql::options::{DefaultDBOptions, DefaultDBPoolOptions};
use crate::sql::types::{SqlxOptionsBuilder, SqlxPoolOptionsBuilder};

const DEFAULT_PORT: u16 = 5432;

pub struct Options {
    app_name: Option<String>,
    db_opts: DefaultDBOptions,
    pool_opts: DefaultDBPoolOptions,
}

impl Options {
    pub fn new(db_opts: DefaultDBOptions, pool_opts: DefaultDBPoolOptions, app_name: Option<String>) -> Result<Self, StorageError> {
        let _ = db_opts.validate()?;
        Ok(Self { db_opts, pool_opts, app_name }) 
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
        let mut pg_pool_opts = PoolOptions::<Self::SqlxDatabase>::new();
        if let Some(max_conns) = &self.pool_opts.max_conns {
            pg_pool_opts =pg_pool_opts 
                .clone()
                .max_connections(max_conns.to_owned());
        }

        if let Some(min_conns) = &self.pool_opts.min_conns {
            pg_pool_opts =pg_pool_opts 
                .clone()
                .min_connections(min_conns.to_owned());
        }

        if let Some(idle) = &self.pool_opts.idle_duration {
            pg_pool_opts = pg_pool_opts.clone().idle_timeout(idle.to_owned());
        }

        if let Some(lifetime) = &self.pool_opts.lifetime_duration {
            pg_pool_opts = pg_pool_opts.clone().max_lifetime(lifetime.to_owned());
        }

        if let Some(acquire) = &self.pool_opts.acquire_timeout {
            pg_pool_opts = pg_pool_opts.clone().acquire_timeout(acquire.to_owned());
        }

       pg_pool_opts 
    }
}
