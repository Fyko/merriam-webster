use super::{
    binding_substitute::BindingSubstitute,
    parenthesized_sense_sequence::ParenthesizedSenseSequence,
    truncated_sense::{TruncatedSense, TruncatedSenseObject},
    Senses,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SenseSequences {
    SenseSequence(Box<SenseSequence>),
    SenseSequences(Vec<SenseSequences>),
}

pub type SenseSequence = Vec<SenseSequenceType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SenseSequenceType {
    Senses(Senses),
    BindingSubstitute(Box<BindingSubstitute>),
    ParenthesizedSenseSequence(ParenthesizedSenseSequence),
    TruncatedSense(TruncatedSense),
    TruncatedSenseObject(TruncatedSenseObject),
}
