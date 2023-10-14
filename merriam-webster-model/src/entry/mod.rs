pub mod sense;

pub mod labels;
pub mod alternate_headwords;
pub mod artwork;
pub mod attribution_of_quote;
pub mod biographical_name_wrap;
pub mod called_also_note;
pub mod cognate_cross_references;
pub mod defined_run_ons;
pub mod directional_cross_reference_section;
pub mod entry;
pub mod etymology;
pub mod first_known_use;
pub mod headword_information;
pub mod homograph;
pub mod inflections;
pub mod meta;
pub mod pronunciations;
pub mod quotation_section;
pub mod run_in;
pub mod short_definitions;
pub mod supplemental_information_note;
pub mod synonym_section;
pub mod tables;
pub mod undefined_run_ons;
pub mod usage_notes;
pub mod usage_section;
pub mod variants;
pub mod verbal_illustrations;

pub use entry::Entry;
pub use sense::Sense;