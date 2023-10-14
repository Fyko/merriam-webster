//! ## 2.7 LABELS
//! A label provides a brief note on the grammatical function, subject area, register, regional usage, or capitalization of a headword, whether generally or in a particular sense.
//!
//! [API Documentation ↗︎]
//!
//! [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.labels

/// ## 2.7.1 FUNCTIONAL LABEL: `FL`
/// The functional label describes the grammatical function of a headword or undefined entry word, such as "noun" or "adjective".
///
/// ### Hierarchical Context
/// Occurs as a top-level member of the dictionary entry, where it describes the hw in the preceding hwi. Also occurs within uros, where it describes the preceding ure.
///
/// ### Display Guidance
/// Typically rendered in italics
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.fl
pub type FunctionalLabel = String;

/// ## 2.7.2 GENERAL LABEL: `LBS`
/// General labels provide information such as whether a headword is typically capitalized, used as an attributive noun, etc. A set of one or more such *la*bels is contained in an `lbs`.
///
/// ### Hierarchical Context
/// Occurs as top-level member of dictionary entry and in `dros`, `sdsense`, `sen`, `sense`, or `uros`.
///
/// ### Display Guidance
/// Typically rendered in italics. If there is a more than one element in the array, separate them with a comma and space.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.lbs
pub type GeneralLabels = Vec<String>;

/// ## 2.7.3 SUBJECT/STATUS LABELS: `SLS`
/// A subject/status label describes the subject area (eg, "computing") or regional/usage
/// status (eg, "British", "formal", "slang") of a headword or a particular sense of a headword.
/// A set of one or more subject/status *la*bels is contained in an sls.
///
/// ### Hierarchical Context
/// Occurs as top-level member of dictionary entry and in `def`, `dros`, `sdsense`, `sen`, `sense`, or `uros`.
///
/// ### Display Guidance
/// Typically rendered in italics. If there is a more than one element in the array, separate with a comma and space.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.sls
pub type SubjectStatusLabels = Vec<String>;

/// ## 2.7.4 PARENTHESIZED SUBJECT/STATUS LABEL: `PSL`
/// A parenthesized subject/status label describes the subject area or regional/usage status (eg, "British") of a headword or other defined term, and is displayed in parentheses. The parenthesized subject/status label is contained in a `psl`.
///
/// ### Hierarchical Context
/// Occurs in `ahws`, `cats`, `dros`, `hwi`, `vi`, or `uros`.
///
/// ### Display Guidance
/// Display within parentheses and in italics.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.psl
pub type ParenthesizedSubjectStatusLabel = String;

/// ## 2.7.5 SENSE-SPECIFIC INFLECTION PLURAL LABEL: `SPL`
/// This label provides information on the grammatical number (eg, singular, plural) of an inflection in a particular sense. A sense-specific inflection **p**lural **la**bel is contained in an `spl`.
///
/// ### Hierarchical Context
/// Occurs in `ins`.
///
/// ### Display Guidance
/// Typically rendered in italics.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.spl
pub type SenseSpecificInflectionPluralLabel = String;

/// ## 2.7.6 SENSE-SPECIFIC GRAMMATICAL LABEL: `SGRAM`
/// This label notes whether a particular sense of a verb is transitive (T) or intransitive (I). The sense-specific grammatical label is contained in an `sgram`.
///
/// ### Hierarchical Context
/// Occurs in `sdsense`, `sen`, `sense`.
///
/// ### Display Guidance
/// Typically displayed within square brackets and in italics.
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.sgram
pub type SenseSpecificGrammaticalLabel = String;
