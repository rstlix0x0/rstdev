use core::time::Duration;

use sqlx::mysql::{MySql, MySqlConnectOptions};
use sqlx::pool::PoolOptions as SqlxPoolOptions;

use rst_common::with_errors::thiserror::{self, Error};

#[derive(Debug, Error, PartialEq)]
pub enum MySQLError {
    #[error("options error: {0}")]
    OptionsError(String),
}

const DEFAULT_PORT: u16 = 3306;

#[derive(Debug, Clone)]
pub struct Options {
    pub host: Option<String>,
    pub username: String,
    pub password: String,
    pub db: String,
    pub port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct PoolOptions {
    pub db_opts: Options,
    pub max_conns: Option<u32>,
    pub min_conns: Option<u32>,
    pub idle_duration: Option<Duration>,
    pub lifetime_duration: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
}

impl Options {
    pub fn validate(&self) -> Result<(), MySQLError> {
        if self.username.is_empty() {
            return Err(MySQLError::OptionsError("username is empty".to_string()));
        }

        if self.password.is_empty() {
            return Err(MySQLError::OptionsError("password is empty".to_string()));
        }

        if self.db.is_empty() {
            return Err(MySQLError::OptionsError("db is empty".to_string()));
        }

        Ok(())
    }

    pub fn to_sqlx_options(&self) -> MySqlConnectOptions {
        let mut mysql_options = MySqlConnectOptions::new()
            .username(&self.username.as_str())
            .database(&self.db.as_str())
            .password(&self.password.as_str());

        if let Some(host) = &self.host {
            mysql_options = mysql_options.host(host.to_owned().as_str());
        }

        match self.port {
            Some(port) => mysql_options.port(port),
            None => mysql_options.port(DEFAULT_PORT),
        }
    }
}

impl PoolOptions {
    pub fn new(db_opts: Options) -> Result<Self, MySQLError> {
        let _ = db_opts.validate()?;
        Ok(Self {
            db_opts,
            max_conns: None,
            min_conns: Some(1),
            idle_duration: None,
            lifetime_duration: None,
            acquire_timeout: None,
        })
    }

    pub fn to_sqlx_pool_options(&self) -> SqlxPoolOptions<MySql> {
        let mut mysql_pool_opts = SqlxPoolOptions::<MySql>::new();
        if let Some(max_conns) = &self.max_conns {
            mysql_pool_opts = mysql_pool_opts
                .clone()
                .max_connections(max_conns.to_owned());
        }

        if let Some(min_conns) = &self.min_conns {
            mysql_pool_opts = mysql_pool_opts
                .clone()
                .min_connections(min_conns.to_owned());
        }

        if let Some(idle) = &self.idle_duration {
            mysql_pool_opts = mysql_pool_opts.clone().idle_timeout(idle.to_owned());
        }

        if let Some(lifetime) = &self.lifetime_duration {
            mysql_pool_opts = mysql_pool_opts.clone().max_lifetime(lifetime.to_owned());
        }

        if let Some(acquire) = &self.acquire_timeout {
            mysql_pool_opts = mysql_pool_opts.clone().acquire_timeout(acquire.to_owned());
        }

        mysql_pool_opts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rst_common::with_tests::table_test::table_test;

    #[test]
    fn test_mysql_options_validation_error() {
        let table = vec![
            (
                (
                    "case with empty username",
                    Options {
                        host: Some("host".to_string()),
                        port: None,
                        username: "".to_string(),
                        password: "password".to_string(),
                        db: "db".to_string(),
                    },
                ),
                MySQLError::OptionsError("username is empty".to_string()),
            ),
            (
                (
                    "case with empty password",
                    Options {
                        host: Some("host".to_string()),
                        port: None,
                        username: "username".to_string(),
                        password: "".to_string(),
                        db: "db".to_string(),
                    },
                ),
                MySQLError::OptionsError("password is empty".to_string()),
            ),
            (
                (
                    "case with empty database",
                    Options {
                        host: Some("host".to_string()),
                        port: None,
                        username: "username".to_string(),
                        password: "password".to_string(),
                        db: "".to_string(),
                    },
                ),
                MySQLError::OptionsError("db is empty".to_string()),
            ),
        ];

        for (test_case, (case_name, input), expected_err) in table_test!(table) {
            test_case
                .given(case_name)
                .when("validate")
                .then("it should be throw an error")
                .assert_eq(input.validate().unwrap_err(), expected_err);
        }
    }

    #[test]
    fn test_pool_options_build_error() {
        let db_opts = Options {
            host: Some("host".to_string()),
            port: None,
            username: "username".to_string(),
            password: "password".to_string(),
            db: "".to_string(),
        };

        let pool_opts = PoolOptions::new(db_opts);
        assert!(pool_opts.is_err());
        assert!(matches!(
            pool_opts.unwrap_err(),
            MySQLError::OptionsError(_)
        ))
    }

    #[test]
    fn test_pool_options_build_success() {
        let db_opts = Options {
            host: Some("host".to_string()),
            port: None,
            username: "username".to_string(),
            password: "password".to_string(),
            db: "db".to_string(),
        };

        let pool_opts = PoolOptions::new(db_opts);
        assert!(!pool_opts.is_err());

        let mut pool = pool_opts.unwrap();
        pool.max_conns = Some(10);
        pool.min_conns = Some(5);
        pool.idle_duration = Some(Duration::from_secs(5));
        pool.acquire_timeout = Some(Duration::from_secs(2));
        pool.lifetime_duration = Some(Duration::from_secs(10));

        let mysql_pool = pool.to_sqlx_pool_options();
        assert_eq!(mysql_pool.get_max_connections(), 10);
        assert_eq!(mysql_pool.get_min_connections(), 5);
        assert_eq!(
            mysql_pool.get_idle_timeout().unwrap(),
            Duration::from_secs(5)
        );
        assert_eq!(mysql_pool.get_acquire_timeout(), Duration::from_secs(2));
        assert_eq!(
            mysql_pool.get_max_lifetime().unwrap(),
            Duration::from_secs(10)
        );
    }
}
