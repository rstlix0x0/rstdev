#![doc = include_str!("../README.md")]

pub mod source;
pub mod types;

mod builder;
mod format;
mod formatter;

pub use builder::Builder;
pub use format::Format;
pub use formatter::StringValue;
