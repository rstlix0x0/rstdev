use sqlx::mysql::{MySql, MySqlConnectOptions};
use sqlx::pool::PoolOptions;

use crate::types::StorageError;

use crate::sql::options::{DefaultDBOptions, DefaultDBPoolOptions, DefaultDBPoolOptionsBuilder};
use crate::sql::types::{SqlxOptionsBuilder, SqlxPoolOptionsBuilder};

const DEFAULT_PORT: u16 = 3306;

pub struct Options {
    db_opts: DefaultDBOptions,
    pool_opts: DefaultDBPoolOptions,
}

impl Options {
    pub fn new(
        db_opts: DefaultDBOptions,
        pool_opts: DefaultDBPoolOptions,
    ) -> Result<Self, StorageError> {
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
        DefaultDBPoolOptionsBuilder::<Self::SqlxDatabase>::new(self.pool_opts.to_owned()).build()
    }
}
