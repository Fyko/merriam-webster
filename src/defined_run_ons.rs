use crate::{
    inflections::Inflections,
    labels::{GeneralLabels, ParenthesizedSubjectStatusLabel, SubjectStatusLabels},
    pronunciations::Pronunciations,
    sense::definition_section::DefinitionSections,
    usage_notes::UsageNotes,
    variants::Variants,
    verbal_illustrations::VerbalIllustrations,
};

pub type DefinedRunOns = Vec<DefinedRunOn>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedRunOn {
    #[serde(rename = "drp")]
    pub value: Option<String>,
    #[serde(rename = "def")]
    pub definitions: DefinitionSections,
    #[serde(rename = "utxt")]
    pub text: Option<Vec<DefinedRunOnText>>,
    #[serde(rename = "ins")]
    pub inflections: Option<Inflections>,
    #[serde(rename = "lbs")]
    pub labels: Option<GeneralLabels>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "psl")]
    pub parenthesized_subect_status_label: Option<ParenthesizedSubjectStatusLabel>,
    #[serde(rename = "sls")]
    pub subject_status_labels: Option<SubjectStatusLabels>,
    #[serde(rename = "vrs")]
    pub variants: Option<Variants>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinedRunOnText {
    VerbalIllustrations(VerbalIllustrations),
    UsageNotes(UsageNotes),
}
