use crate::types::SourceFormatter;

/// `Value` is a main object that will be used as
/// value type of `String`.
///
/// This object MUST implement [`SourceFormatter`]
#[derive(Clone)]
pub struct Value {
    input: String,
}

impl Value {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl<'a> SourceFormatter<'a, String> for Value {
    fn get_source_value(&'a self) -> String {
        self.input.clone()
    }
}
