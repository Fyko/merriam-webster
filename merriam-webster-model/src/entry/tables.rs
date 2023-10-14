#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    #[serde(rename = "tableid")]
    pub id: String,
    #[serde(rename = "displayname")]
    pub display_name: String,
}
