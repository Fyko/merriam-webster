use super::{labels::ParenthesizedSubjectStatusLabel, pronunciations::Pronunciations};

pub type AlternateHeadwords = Vec<AlternateHeadword>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateHeadword {
    #[serde(rename = "hw")]
    pub value: String,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "psl")]
    pub parenthesized_subject_status_label: Option<ParenthesizedSubjectStatusLabel>,
}
