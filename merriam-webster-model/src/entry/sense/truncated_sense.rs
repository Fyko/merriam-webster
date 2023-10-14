use super::sense_number::SenseNumber;
use crate::entry::{
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    pronunciations::Pronunciations,
    variants::Variants,
};

pub type TruncatedSense = (TruncatedSenseKey, TruncatedSenseObject);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncatedSenseObject {
    et: Option<Etymology>,
    ins: Option<Inflections>,
    lbs: Option<GeneralLabels>,
    prs: Option<Pronunciations>,
    sgram: Option<SenseSpecificGrammaticalLabel>,
    sls: Option<SubjectStatusLabels>,
    sn: Option<SenseNumber>,
    vrs: Option<Variants>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TruncatedSenseKey {
    #[serde(rename = "sen")]
    Key,
}
