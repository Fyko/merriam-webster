use super::{binding_substitute::BindingSubstitute, Sense, Senses};

pub type ParenthesizedSenseSequence = (
    ParenthesizedSenseSequenceKey,
    Vec<InnerParenthesizedSenseSequence>,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerParenthesizedSenseSequence {
    Sense(Sense),
    Senses(Vec<Senses>),
    BindingSubstitute(BindingSubstitute),
    BindingSubstitutes(Vec<BindingSubstitute>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParenthesizedSenseSequenceKey {
    #[serde(rename = "pseq")]
    Key,
}
