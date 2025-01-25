#[derive(Debug, Clone)]
pub enum Value {
    DWord(u32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DWord(v) => write!(f, "{v}"),
        }
    }
}
