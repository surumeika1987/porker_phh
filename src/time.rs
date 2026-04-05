use std::fmt::{self, Display, Formatter};
use time::Time;
use crate::error::Error;

/// Represents time in the PHH format.
///
/// This wrapper around `time::Time` provides serialization/deserialization
/// support for TOML-based PHH files.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PHHTime(pub Time);

impl From<toml::value::Time> for PHHTime {
    fn from(value: toml::value::Time) -> Self {
        Self(Time::from_hms_nano(
                value.hour,
                value.minute,
                value.second.unwrap_or_default(),
                value.nanosecond.unwrap_or_default(),
            ).unwrap())
    }
}

impl TryFrom<toml::Value> for PHHTime {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        if let toml::Value::Datetime(dt) = value {
            if let Some(t) = dt.time {
                return Ok(PHHTime::from(t));
            }
        }
        Err(Error::ParseError(
                format!(r#"Invalid time "{}". expect TOML local time."#, value.to_string())
        ))
    }
}

impl Display for PHHTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
