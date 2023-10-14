use crate::{labels::SenseSpecificInflectionPluralLabel, pronunciations::Pronunciations};

pub type Inflections = Vec<Inflection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inflection {
    #[serde(rename = "if")]
    pub value: String,
    #[serde(rename = "ifc")]
    pub cutback: Option<String>,
    #[serde(rename = "il")]
    pub label: Option<String>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "spl")]
    pub sense_specific_inflection_plural_label: Option<SenseSpecificInflectionPluralLabel>,
}
