pub mod binding_substitute;
pub mod defining_text;
pub mod definition_section;
pub mod divided_sense;
pub mod parenthesized_sense_sequence;
pub mod sense_number;
pub mod sense_sequence;
pub mod truncated_sense;
pub mod verb_divider;

use super::{
    etymology::Etymology,
    inflections::Inflections,
    labels::{GeneralLabels, SenseSpecificGrammaticalLabel, SubjectStatusLabels},
    pronunciations::Pronunciations,
    variants::Variants,
};

use self::{defining_text::DefiningText, divided_sense::DividedSense, sense_number::SenseNumber};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Senses {
    Sense(Box<Sense>),
    Senses(Vec<Senses>),
}

pub type Sense = (SenseKey, SenseObject);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SenseKey {
    #[serde(rename = "sense")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseObject {
    dt: DefiningText,
    et: Option<Etymology>,
    ins: Option<Inflections>,
    lbs: Option<GeneralLabels>,
    prs: Option<Pronunciations>,
    sdsense: Option<DividedSense>,
    sgram: Option<SenseSpecificGrammaticalLabel>,
    sls: Option<SubjectStatusLabels>,
    sn: Option<SenseNumber>,
    vrs: Option<Variants>,
}
