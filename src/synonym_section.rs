use crate::verbal_illustrations::VerbalIllustrations;

pub type SynonymSection = Vec<SynonymDiscussion>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymDiscussion {
    #[serde(rename = "pl")]
    pub label: String,
    #[serde(rename = "pt")]
    pub text: Vec<SynonymTextType>,
    #[serde(rename = "sarefs")]
    pub see_in_addition: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SynonymTextType {
    Text(TextSynonymTextType),
    VerbalIllustrations(VerbalIllustrations),
}

pub type TextSynonymTextType = (TextSynonymTextTypeKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextSynonymTextTypeKey {
    #[serde(rename = "text")]
    Key,
}
