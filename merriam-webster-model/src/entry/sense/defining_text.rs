use crate::entry::{
    biographical_name_wrap::BiographicalNameWrap, called_also_note::CalledAlsoNote, run_in::RunIn,
    supplemental_information_note::SupplementalInformationNote, usage_notes::UsageNotes,
    verbal_illustrations::VerbalIllustrations,
};

pub type DefiningText = Vec<DefiningTextType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefiningTextType {
    DefiningTextObject(DefiningTextObject),
    BiographicalNameWrap(BiographicalNameWrap),
    CalledAlsoNote(CalledAlsoNote),
    RunIn(RunIn),
    SupplementalInformationNote(SupplementalInformationNote),
    UsageNotes(UsageNotes),
    VerbalIllustrations(VerbalIllustrations),
}

pub type DefiningTextObject = (DefiningTextObjectKey, String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DefiningTextObjectKey {
    #[serde(rename = "text")]
    Key,
}
