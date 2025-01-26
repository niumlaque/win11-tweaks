use crate::win;

#[derive(Debug)]
pub struct OptionCandidate {
    pub value: String,
    pub description: String,
}

impl OptionCandidate {
    pub fn new(value: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug)]
pub struct RegistryEditOption {
    pub id: u64,
    pub label: String,
    pub def: win::reg::RegDef,
    pub values: Vec<OptionCandidate>,
}
