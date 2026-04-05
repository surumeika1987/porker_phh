/// Represents errors that occur during PHH parsing and validation.
///
/// This enum provides a unified way to handle errors that arise when parsing
/// or validating PHH (Poker Hand History) format data.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Error {
    /// A parsing error with a descriptive message.
    ParseError(String),
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::ParseError(value.message().to_string())
    }
}
