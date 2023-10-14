//! ## 2.9 COGNATE CROSS-REFERENCES: `CXS`
//! When a headword is a less common spelling of another word with the same meaning, there will be a cognate cross-reference pointing to the headword with the more common spelling.
//! A set of cognate cross-references is contained in a `cxs`.
//!
//! ### Hierarchical Context
//!
//! Top-level member of dictionary entry.
//!
//! ### Display Guidance
//!
//! The `cxt` cross-reference target is typically displayed in smallcaps. If `cxr` is present, do not display but use in forming cross-reference hyperlink as described below.
//!
//! The `cxl` label is typically displayed in italics and should be followed by a space.
//!
//! A `cxn` is typically displayed in normal font and should be preceded by a space.
//!
//! If the `cxs` array has more than one element, separate them by a comma and space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognateCrossReferenceTarget {
    /// cognate cross-reference label
    #[serde(rename = "cxl")]
    pub label: Option<String>,
    /// when present, use as cross-reference target ID for immediately preceding cxt
    #[serde(rename = "cxr")]
    pub reference: Option<String>,
    /// provides hyperlink text in all cases, and cross-reference target ID when no immediately following cxr
    #[serde(rename = "cxt")]
    pub hyperlink_text: Option<String>,
    /// sense number of cross-reference target
    pub sense_number: Option<String>,
}

pub type CognateCrossReferenceTargets = Vec<CognateCrossReferenceTarget>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognateCrossReference {
    /// cognate cross-reference label
    #[serde(rename = "cxl")]
    pub label: String,
    /// one or more cognate cross-reference targets
    #[serde(rename = "cxtis")]
    pub targets: CognateCrossReferenceTargets,
}

pub type CognateCrossReferences = Vec<CognateCrossReference>;
