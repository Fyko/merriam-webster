use crate::{labels::ParenthesizedSubjectStatusLabel, pronunciations::Pronunciations};

pub type CalledAlsoNote = (CalledAlsoNoteKey, InnerCalledAlsoNote);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalledAlsoNoteKey {
    #[serde(rename = "ca")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerCalledAlsoNote {
    pub intro: String,
    #[serde(rename = "cats")]
    pub targets: Vec<CalledAlsoTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalledAlsoTarget {
    #[serde(rename = "cat")]
    pub value: String,
    #[serde(rename = "catref")]
    pub reference: Option<String>,
    #[serde(rename = "pn")]
    pub parenthesized_number: Option<String>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "pls")]
    pub parenthesized_subject_status_labels: Option<ParenthesizedSubjectStatusLabel>,
}
