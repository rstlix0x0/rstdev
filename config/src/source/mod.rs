//! A `source` module used to store multiple implementation of `SourceParser`
//!
//! We just prevent to re-export all source adapter implementer to simplify our `Builder`
mod file;

/// `from_file` used to build `File` adapter which is an adapter that read
/// configuration from a physical file.
pub fn from_file(filepath: String) -> file::File {
    file::File::new(filepath)
}
