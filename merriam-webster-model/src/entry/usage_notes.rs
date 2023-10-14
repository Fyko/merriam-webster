use super::{run_in::RunIn, verbal_illustrations::VerbalIllustrations};

pub type UsageNotes = (UsageNotesKey, Vec<UsageNote>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsageNotesKey {
    #[serde(rename = "uns")]
    Key,
}

pub type UsageNote = Vec<UsageNoteType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsageNoteType {
    Text(TextUsageNote),
    RunIn(RunIn),
    VerbalIllustrations(VerbalIllustrations),
}

pub type TextUsageNote = (UsageNoteKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsageNoteKey {
    #[serde(rename = "text")]
    Key,
}
