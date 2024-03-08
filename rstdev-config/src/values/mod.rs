//! A `values` is an object wrapper used to save a config data from the parser
//!
//! For now it just provide a [`StringValue`] type which is an object wrapper to save a string input type
mod string;
mod tuple;

pub use string::Value as StringValue;
pub use tuple::Value as TupleValue;
