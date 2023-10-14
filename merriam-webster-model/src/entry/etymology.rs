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
///

pub type Etymology = Vec<EtymologyType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EtymologyType {
    Content(EtymologyContent),
    SupplementalNote(EtymologySupplementalNote),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EtymologyContentKey {
    #[serde(rename = "text")]
    Key,
}

pub type EtymologyContent = (EtymologyContentKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EtymologySupplementalNoteKey {
    #[serde(rename = "et_snote")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InnerEtymologySupplementalNoteKey {
    #[serde(rename = "t")]
    Key,
}

pub type EtymologySupplementalNote = (
    EtymologySupplementalNoteKey,
    Vec<(InnerEtymologySupplementalNoteKey, String)>,
);
