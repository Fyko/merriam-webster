//! https://dictionaryapi.com/products/json#sec-2.prs
pub type Pronunciations = Vec<Pronunciation>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pronunciation {
    /// written pronunciation in Merriam-Webster format
    pub mw: Option<String>,
    /// pronunciation label before pronunciation
    #[serde(rename = "l")]
    pub label_before: Option<String>,
    /// pronunciation label after pronunciation
    #[serde(rename = "l2")]
    pub label_after: Option<String>,
    /// punctuation to separate pronunciation objects
    #[serde(rename = "pun")]
    pub punctuation: Option<String>,
    /// audio playback information
    pub sound: Option<PronunciationSound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronunciationSound {
    pub audio: String,
}

impl PronunciationSound {
    /// Returns the URL of the sound file.
    ///
    /// - `language_code` is usually `us` but can be `es`.
    /// - `country_code` is ucually `us` but can be `me`.
    /// - `format` can be `mp3`, `wav` or `ogg`.
    pub fn to_url(&self, language_code: String, country_code: String, format: String) -> String {
        let sub_directory = if self.audio.starts_with("bix") {
            // if audio begins with "bix", the subdirectory should be "bix",
            "bix"
        } else if self.audio.starts_with("gg") {
            // if audio begins with "gg", the subdirectory should be "gg",
            "gg"
        } else if self
            .audio
            .starts_with(|c: char| c.is_numeric() || c.is_ascii_punctuation())
        {
            // if audio begins with a number or punctuation (eg, "_"), the subdirectory should be "number",
            "number"
        } else {
            // otherwise, the subdirectory is equal to the first letter of audio.
            &self.audio[0..1]
        };

        format!("https://media.merriam-webster.com/audio/prons/{language_code}/{country_code}/{format}/{sub_directory}/{}.{format}", self.audio)
    }
}
