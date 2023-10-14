#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// unique entry identifier within a particular dictionary data set, used in cross-references pointing to the entry.
    /// It consists of the headword, and in homograph entries, an appended colon and homograph number.
    pub id: String,
    /// universally unique identifier
    pub uuid: String,
    /// a 9-digit code which may be used to sort entries in the proper dictionary order if alphabetical display is needed
    pub sort: String,
    /// source data set for entry—ignore
    pub src: String,
    /// indicates the section the entry belongs to in print, where "alpha" indicates the main alphabetical section, "biog" biographical, "geog" geographical, and "fw&p" the foreign words & phrases section.
    pub section: String,
    /// lists all of the entry's headwords, variants, inflections, undefined entry words, and defined run-on phrases. Each stem string is a valid search term that should match this entry.
    pub stems: Vec<String>,
    /// true if there is a label containing "offensive" in the entry; otherwise, false.
    pub offensive: bool,
}
