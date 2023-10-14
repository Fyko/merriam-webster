/// ## 2.14 BIOGRAPHICAL NAME WRAP: `BNW`
/// A biographical name wrap groups together personal name, surname, and alternate name information within a biographical entry.
/// The *b*iographical *n*ame *w*rap is contained in a `bnw`.
///
/// ### Hierarchical Context
/// Occurs in `dt`.
///
/// ### Display Guidance
/// A `bnw` is displayed inline and followed by a comma and space.
///
/// A `pname` or `sname` is typically displayed in normal font.
///
/// An `altname` is typically displayed in italics.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.bnw

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiographicalNameWrapKey {
    #[serde(rename = "bnw")]
    Key,
}

pub type BiographicalNameWrap = (BiographicalNameWrapKey, InnerBiographicalNameWrap);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerBiographicalNameWrap {
    /// personal or first name
    #[serde(rename = "pname")]
    pub first_name: Option<String>,
    /// surname
    #[serde(rename = "sname")]
    pub surname: Option<String>,
    /// alternate name such as pen name, nickname, title, etc.
    #[serde(rename = "altname")]
    pub alternate_name: Option<String>,
}
