use crate::pronunciations::Pronunciations;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadwordInformation {
    #[serde(rename = "hw")]
    pub value: String,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
}
