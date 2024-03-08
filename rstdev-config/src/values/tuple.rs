use crate::types::SourceFormatter;

/// `Value` is a main object used to store a collection tuple
/// of `String`
///
/// This object MUST implement [`SourceFormatter`]
#[derive(Debug)]
pub struct Value {
    input: Vec<(String, String)>,
}

impl Value {
    pub fn new(input: Vec<(String, String)>) -> Self {
        Self { input }
    }
}

impl<'a> SourceFormatter<'a, Vec<(String, String)>> for Value {
    fn get_source_value(&'a self) -> Vec<(String, String)> {
        self.input.to_owned()
    }
}
