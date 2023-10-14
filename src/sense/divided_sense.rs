use crate::{
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    pronunciations::Pronunciations,
    variants::Variants,
};

use super::defining_text::DefiningText;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DividedSense {
    #[serde(rename = "sd")]
    pub value: String,
    #[serde(rename = "dt")]
    pub defining_text: DefiningText,
    #[serde(rename = "et")]
    pub etymology: Option<Etymology>,
    #[serde(rename = "ins")]
    pub inflections: Option<Inflections>,
    #[serde(rename = "lbs")]
    pub labels: Option<GeneralLabels>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "sgram")]
    pub sense_specific_grammatical_label: Option<SenseSpecificGrammaticalLabel>,
    #[serde(rename = "sls")]
    pub subect_status_labels: Option<SubjectStatusLabels>,
    #[serde(rename = "vrs")]
    pub variants: Option<Variants>,
}
