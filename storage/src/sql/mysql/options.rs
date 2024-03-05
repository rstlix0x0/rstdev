use sqlx::mysql::{MySql, MySqlConnectOptions};
use sqlx::pool::PoolOptions;

use crate::types::StorageError;

use crate::sql::options::{DefaultDBOptions, DefaultDBPoolOptions};
use crate::sql::types::{SqlxOptionsBuilder, SqlxPoolOptionsBuilder};

const DEFAULT_PORT: u16 = 3306;

pub struct Options {
    db_opts: DefaultDBOptions,
    pool_opts: DefaultDBPoolOptions,
}

impl Options {
    pub fn new(db_opts: DefaultDBOptions, pool_opts: DefaultDBPoolOptions) -> Result<Self, StorageError> {
        let _ = db_opts.validate()?;
        Ok(Self { db_opts, pool_opts }) 
    }
}

impl SqlxOptionsBuilder for Options {
    type SqlxOptionType = MySqlConnectOptions;

    fn common_options(&self) -> Self::SqlxOptionType {
        let mut mysql_options = MySqlConnectOptions::new()
            .username(&self.db_opts.username.as_str())
            .database(&self.db_opts.db.as_str())
            .password(&self.db_opts.password.as_str());

        if let Some(host) = &self.db_opts.host {
            mysql_options = mysql_options.host(host.to_owned().as_str());
        }

        match self.db_opts.port {
            Some(port) => mysql_options.port(port),
            None => mysql_options.port(DEFAULT_PORT),
        }
    }
}

impl SqlxPoolOptionsBuilder for Options {
    type SqlxDatabase = MySql;

    fn pool_options(&self) -> PoolOptions<Self::SqlxDatabase> {
        let mut mysql_pool_opts = PoolOptions::<Self::SqlxDatabase>::new();
        if let Some(max_conns) = &self.pool_opts.max_conns {
            mysql_pool_opts = mysql_pool_opts
                .clone()
                .max_connections(max_conns.to_owned());
        }

        if let Some(min_conns) = &self.pool_opts.min_conns {
            mysql_pool_opts = mysql_pool_opts
                .clone()
                .min_connections(min_conns.to_owned());
        }

        if let Some(idle) = &self.pool_opts.idle_duration {
            mysql_pool_opts = mysql_pool_opts.clone().idle_timeout(idle.to_owned());
        }

        if let Some(lifetime) = &self.pool_opts.lifetime_duration {
            mysql_pool_opts = mysql_pool_opts.clone().max_lifetime(lifetime.to_owned());
        }

        if let Some(acquire) = &self.pool_opts.acquire_timeout {
            mysql_pool_opts = mysql_pool_opts.clone().acquire_timeout(acquire.to_owned());
        }

        mysql_pool_opts
    }
}
