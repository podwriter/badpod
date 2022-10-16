use serde::{Deserialize, Deserializer};

//// mod category;
//// pub use category::{CategoryName, SubcategoryName};

//// #[derive(Debug, Deserialize, PartialEq, Default)]
//// pub struct Category {
////     #[serde(rename = "$attr:text")]
////     pub text: Option<CategoryName>,
////
////     #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category")]
////     pub subcategory: Option<Subcategory>,
//// }
////
//// #[derive(Debug, Deserialize, PartialEq, Default)]
//// pub struct Subcategory {
////     #[serde(rename = "$attr:text")]
////     pub text: Option<SubcategoryName>,
//// }

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Image {
    #[serde(rename = "$attr:href")]
    pub href: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Owner {
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:email")]
    pub email: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:name")]
    pub name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Yes {
    Yes,
    Other(String),
}

impl<'de> Deserialize<'de> for Yes {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.as_str() {
            "Yes" => Ok(Yes::Yes),
            _ => Ok(Yes::Other(s)),
        }
    }
}

//// #[derive(Debug, Deserialize_enum_str, PartialEq)]
//// #[serde(rename_all = "snake_case")]
//// pub enum PodcastType {
////     Episodic,
////     Serial,
////     #[serde(other)]
////     Other(String),
//// }
