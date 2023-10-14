use super::SenseObject;

pub type BindingSubstitute = (BindingSubstituteKey, InnerBindingSubstitute);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerBindingSubstitute {
    pub sense: SenseObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindingSubstituteKey {
    #[serde(rename = "bs")]
    Key,
}
