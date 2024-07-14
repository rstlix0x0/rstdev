//! `rocksdb` provide an implementation of `Storage` for the `RocksDB`, built on top
//! of `rust-rocksdb` package

pub mod db;
pub mod executor;
pub mod options;
pub mod types;

pub mod lib {
    pub use rust_rocksdb;
}
