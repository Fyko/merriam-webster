use super::{sense_sequence::SenseSequences, verb_divider::VerbDivider};

pub type DefinitionSections = Vec<DefinitionSection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionSection {
    #[serde(rename = "vd")]
    pub verb_divider: Option<VerbDivider>,
    #[serde(rename = "sseq")]
    pub sense_sequence: SenseSequences,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_big_sense() {
        let data = serde_json::json!({
          "sseq": [
            [
              [
                "sense",
                {
                  "dt": [
                    [
                      "text",
                      "{bc}one that {a_link|creates} usually by bringing something new or original into being"
                    ]
                  ],
                  "sdsense": {
                    "sd": "especially",
                    "vrs": [
                      {
                        "va": "Creator"
                      }
                    ],
                    "dt": [
                      [
                        "text",
                        "{bc}{sx|god||1}"
                      ]
                    ]
                  }
                }
              ]
            ]
          ]
        });

        let result: Result<DefinitionSection, _> = serde_path_to_error::deserialize(data);
        let _ = match result {
            Ok(res) => res,
            Err(err) => {
                let message = err.to_string();

                panic!("{message}")
            }
        };
    }
}
