//! `sql` module provide basic implementation of `Storage` that build on top of `sqlx` library
//!
//! For now it only support two types of sql databases:
//! - Mysql
//! - Postgres

pub mod options;
pub mod types;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgresql")]
pub mod postgresql;
