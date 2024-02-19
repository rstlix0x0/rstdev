pub mod file;

pub fn from_file(filepath: String) -> file::File {
    file::File::new(filepath)
}
