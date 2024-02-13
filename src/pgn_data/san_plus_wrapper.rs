use pgn_reader::SanPlus;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, str::FromStr};

/// A wrapper around pgn_reader's SanPlus struct that implements Serialize and Deserialize.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SanPlusWrapper(pub SanPlus);

impl Display for SanPlusWrapper {
    /// Formats the SanPlusWrapper as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SanPlusWrapper {
    type Err = std::io::Error;

    /// Parses a string into a SanPlusWrapper.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let san_plus = SanPlus::from_str(s).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse string into SanPlusWrapper",
            )
        })?;
        Ok(SanPlusWrapper(san_plus))
    }
}

impl Serialize for SanPlusWrapper {
    /// Serializes the SanPlusWrapper into a string.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SanPlusWrapper {
    /// Deserializes a string into a SanPlusWrapper.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let san_plus = SanPlus::from_str(&s).map_err(Error::custom)?;
        Ok(SanPlusWrapper(san_plus))
    }
}
