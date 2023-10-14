use crate::attribution_of_quote::AttributionOfQuote;

pub type VerbalIllustrations = (VerbalIllustrationsKey, VerbalIllustration);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbalIllustrationsKey {
    #[serde(rename = "vis")]
    Key,
}

pub type VerbalIllustration = Vec<InnerVerbalIllustration>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerVerbalIllustration {
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "aq")]
    pub attribution_of_quote: Option<AttributionOfQuote>,
}
