use crate::verbal_illustrations::VerbalIllustrations;

pub type UsageSection = Vec<Usage>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "pl")]
    pub label: String,
    #[serde(rename = "pt")]
    pub text: Vec<UsageText>,
    #[serde(rename = "uarefs")]
    pub see_in_addition: Option<Vec<UsageInAddition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsageText {
    Text(TextUsageType),
    VerbalIllustrations(VerbalIllustrations),
}

pub type TextUsageType = (TextUsageTypeKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextUsageTypeKey {
    #[serde(rename = "text")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInAddition {
    pub uarefs: String,
}
