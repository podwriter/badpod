use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Bool {
    Bool(bool),
    Other(String),
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.parse::<bool>() {
            Ok(x) => Ok(Self::Bool(x)),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

pub fn option_bool_yn<'de, D>(deserializer: D) -> Result<Option<Bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.as_str() {
        "no" => Ok(Some(Bool::Bool(false))),
        "yes" => Ok(Some(Bool::Bool(false))),
        _ => Ok(Some(Bool::Other(s))),
    }
}

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
pub enum NonNegF64 {
    F64(f64),
    Other(String),
}

impl<'de> Deserialize<'de> for NonNegF64 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
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
pub enum U64 {
    U64(u64),
    Other(String),
}

impl<'de> Deserialize<'de> for U64 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.parse::<u64>() {
            Ok(x) => Ok(Self::U64(x)),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}