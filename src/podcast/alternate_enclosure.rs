use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};


#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum IntegrityType {
    #[strum(serialize = "sri")]
    SRI,
    #[strum(serialize = "pgp-signature")]
    PGP,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for IntegrityType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}