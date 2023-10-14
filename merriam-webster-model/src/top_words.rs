//! Models for the top words endpoint.

/// The response from the top words endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIGetTopWordsJSONResponse {
	/// The inner data of the response.
	pub data: APIGetTopWordsData,
	/// The status of the response.
	pub messages: String,
}

/// The inner data of the response from the top words endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIGetTopWordsData {
	/// The last time the top words were updated.
	pub timestamp: String,
	/// The current top words.
	pub words: Vec<String>,
}
