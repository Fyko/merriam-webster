use crate::attribution_of_quote::AttributionOfQuote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "aq")]
    pub attribution_of_quote: Option<AttributionOfQuote>,
}

pub type QuotationSection = Vec<Quote>;
