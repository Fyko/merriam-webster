#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artwork {
    #[serde(rename = "artid")]
    pub id: String,
    #[serde(rename = "capt")]
    pub caption: Option<String>,
}
