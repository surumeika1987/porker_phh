use crate::error::Error;

/// Represents custom field values in PHH format.
///
/// PHH files support custom fields prefixed with `_`, which can contain
/// various data types. This enum handles all supported custom value types.
#[derive(Debug, PartialEq, Clone)]
pub enum PHHCustomValue {
    /// A string value.
    String(String),
    /// A floating-point number.
    Float(f64),
    /// An integer value.
    Intger(i64),
    /// A boolean value.
    Boolean(bool),
    /// An array of custom values.
    Array(Vec<PHHCustomValue>),
}

impl TryFrom<toml::Value> for PHHCustomValue {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::String(string) => Ok(PHHCustomValue::String(string)),
            toml::Value::Integer(integer) => Ok(PHHCustomValue::Intger(integer)),
            toml::Value::Float(float) => Ok(PHHCustomValue::Float(float)),
            toml::Value::Boolean(bool) => Ok(PHHCustomValue::Boolean(bool)),
            toml::Value::Array(array) => {
                Ok(PHHCustomValue::Array(
                        array.into_iter()
                        .map(|v| Self::try_from(v))
                        .collect::<Result<Vec<PHHCustomValue>, Error>>()?))
            }
            toml::Value::Datetime(dt) => Ok(PHHCustomValue::String(dt.to_string())),
            toml::Value::Table(_) => Err(Error::ParseError("Custom field table parsing is not yet implemented.".to_string())),
        }
    }
}
