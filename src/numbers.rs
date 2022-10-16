use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum NonNegNumber {
    U64(u64),
    F64(f64),
    Other(String),
}

impl<'de> Deserialize<'de> for NonNegNumber {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        s = match s.parse::<u64>() {
            Ok(x) => return Ok(Self::U64(x)),
            Err(_) => s,
        };

        match s.parse::<f64>() {
            Ok(x) => {
                if x < 0.0 {
                    return Ok(Self::Other(s));
                }
                Ok(Self::F64(x))
            }
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Float {
    Float(f32),
    Other(String),
}

pub fn option_float<'de, D>(deserializer: D) -> Result<Option<Float>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<f32>() {
        Ok(number) => Ok(Some(Float::Float(number))),
        _ => Ok(Some(Float::Other(s))),
    }
}

#[derive(Debug, PartialEq)]
pub enum U64 {
    U64(u64),
    Other(String),
}

pub fn option_u64<'de, D>(deserializer: D) -> Result<Option<U64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<u64>() {
        Ok(number) => Ok(Some(U64::U64(number))),
        _ => Ok(Some(U64::Other(s))),
    }
}
