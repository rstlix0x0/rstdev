//! A `parser` module used to store multiple implementation of `SourceParser`
//!
//! We just prevent to re-export all source adapter implementer to simplify our `Builder`

mod env;
mod file;

/// `from_file` used to build `File` adapter which is an adapter that read
/// configuration from a physical file.
pub fn from_file(filepath: String) -> file::File {
    file::File::new(filepath)
}

/// `from_env` used to build `Env` adapter which is an adapter that read all
/// environment variables
pub fn from_env(prefix: String) -> env::Env {
    env::Env::new(prefix)
}
