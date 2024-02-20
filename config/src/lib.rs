#![doc = include_str!("../README.md")]

mod builder;
mod format;
mod types;

pub mod source;

pub use builder::Builder;
pub use format::Format;
pub use types::{ConfigError, SourceParser};
