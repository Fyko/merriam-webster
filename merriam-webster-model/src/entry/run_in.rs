pub type RunIn = (RunInKey, Vec<RunInType>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunInKey {
    #[serde(rename = "ri")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunInType {
    Wrap(RunInWrap),
    InterveningText(RunInInterveningText),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunInWrapKey {
    #[serde(rename = "riw")]
    Key,
}

pub type RunInWrap = (RunInWrapKey, Vec<RunInEntryWord>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunInInterveningTextKey {
    #[serde(rename = "text")]
    Key,
}

pub type RunInInterveningText = (RunInInterveningTextKey, String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunInEntryWord {
    #[serde(rename = "rie")]
    pub word: String,
}
