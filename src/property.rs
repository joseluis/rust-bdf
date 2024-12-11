/// A `Font` property.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Property {
    ///
    String(String),

    ///
    Integer(i64),
}

impl Property {
    /// Parse a property string.
    pub fn parse(string: &str) -> Property {
        if string.starts_with('"') {
            Property::String(extract(string))
        } else if let Ok(int) = string.parse() {
            Property::Integer(int)
        } else {
            Property::String(string.into())
        }
    }
}

pub fn extract(string: &str) -> String {
    string[1..string.len() - 1].replace("\"\"", "\"")
}
