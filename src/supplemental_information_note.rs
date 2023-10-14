use crate::{run_in::RunIn, verbal_illustrations::VerbalIllustrations};

pub type SupplementalInformationNote = (
    SupplementalInformationNoteKey,
    Vec<SupplementalInformationNoteType>,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplementalInformationNoteKey {
    #[serde(rename = "snote")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SupplementalInformationNoteType {
    Text(TextSupplementalInformationNote),
    RunIn(RunIn),
    VerbalIllustrations(VerbalIllustrations),
}

pub type TextSupplementalInformationNote = (TextSupplementalInformationNoteKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextSupplementalInformationNoteKey {
    #[serde(rename = "t")]
    Key,
}
