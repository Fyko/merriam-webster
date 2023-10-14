use crate::{labels::SenseSpecificInflectionPluralLabel, pronunciations::Pronunciations};

pub type Variants = Vec<Variant>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    #[serde(rename = "va")]
    pub name: String,
    #[serde(rename = "vl")]
    pub label: Option<String>,
    #[serde(rename = "prs")]
    pub pronunciation: Option<Pronunciations>,
    #[serde(rename = "spl")]
    pub spl: Option<SenseSpecificInflectionPluralLabel>,
}
