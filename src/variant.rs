use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
use serde::de::{Visitor, Unexpected};
use crate::error::Error;

/// Represents different poker game variants.
///
/// This enum covers the major poker variants supported by the PHH format,
/// including standard variants and mixed games.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Variant {
    /// Fixed Limit Texas Hold'em
    FT,
    /// No Limit Texas Hold'em
    NT,
    /// No Limit Seven Card Stud
    NS,
    /// Pot Limit Omaha
    PO,
    /// Fixed Limit Omaha Hi/Lo with Eights or Better
    FO8,
    /// Fixed Limit Seven Card Stud
    F7S,
    /// Fixed Limit Seven Card Stud Hi/Lo with Eights or Better
    F7S8,
    /// Fixed Limit Razz (Seven Card Stud Low)
    FR,
    /// No Limit 2-7 Lowball Single Draw
    N2L1D,
    /// Fixed Limit 2-7 Triple Draw Lowball
    F2L3D,
    /// Fixed Limit Badugi
    FB,
}

impl FromStr for Variant {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FT" => Ok(Variant::FT),
            "NT" => Ok(Variant::NT),
            "NS" => Ok(Variant::NS),
            "PO" => Ok(Variant::PO),
            "FO/8" => Ok(Variant::FO8),
            "F7S" => Ok(Variant::F7S),
            "F7S/8" => Ok(Variant::F7S8),
            "FR" => Ok(Variant::FR),
            "N2L1D" => Ok(Variant::N2L1D),
            "F2L3D" => Ok(Variant::F2L3D),
            "FB" => Ok(Variant::FB),
            _ => Err(Error::ParseError(format!(r#"Invalid variant "{}". expect PHH variant."#, s))),
        }
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let variant_str = match self {
            Variant::FT => "FT",
            Variant::NT => "NT",
            Variant::NS => "NS",
            Variant::PO => "PO",
            Variant::FO8 => "FO/8",
            Variant::F7S => "F7S",
            Variant::F7S8 => "F7S/8",
            Variant::FR => "FR",
            Variant::N2L1D => "N2L1D",
            Variant::F2L3D => "F2L3D",
            Variant::FB => "FB",
        };
        write!(f, "{}", variant_str)
    }
}

impl<'de> Deserialize<'de> for Variant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        struct VariantVisitor;

        impl<'de> Visitor<'de> for VariantVisitor {
            type Value = Variant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PHH variant")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> 
                where
                    E: de::Error,
            {
                match Variant::from_str(v) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self)),
                }
            }
        }

        deserializer.deserialize_str(VariantVisitor)
    }
}
