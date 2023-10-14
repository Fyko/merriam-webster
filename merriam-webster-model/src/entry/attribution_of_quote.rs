use serde::{Deserialize, Serialize};

/// ## 2.12 ATTRIBUTION OF QUOTE: `AQ`
/// Direct quotes are used in both verbal illustrations and end-of-entry quotation sections.
/// Information about the attribution (the author, publication, date, etc.) of a particular *q*uote is contained in an `aq`.
///
/// ### Hierarchical Context
/// Occurs in quotes, vis.
///
/// ### Display Guidance
/// The `aq` is typically preceded by an em-dash.
///
/// Each instance of `auth`, `source`, `aqdate`, should be followed by a comma and space.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.aq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionOfQuote {
    /// name of author
    #[serde(rename = "auth")]
    pub author: Option<String>,
    /// source work for quote
    pub source: Option<String>,
    /// publication date of quote
    pub aqdate: Option<String>,
    /// further detail on quote source (eg, name of larger work in which an essay is found)
    pub subsource: Option<AttributionOfQuoteSubsource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionOfQuoteSubsource {
    pub source: Option<String>,
    /// publication date of quote
    pub aqdate: Option<String>,
}
