use std::fmt::Display;

#[derive(Debug)]
pub enum LabelError {
    NotFound,
    DuplicatedLabelName,
    Unexpected(anyhow::Error),
}

impl Into<String> for LabelError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Display for LabelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelError::NotFound => write!(f, "Label not found"),
            LabelError::DuplicatedLabelName => write!(f, "Label name already exists"),
            LabelError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
