use std::fmt::Display;

#[derive(Debug)]
pub enum PostError {
    NotFound,
    InvalidSemanticId,
    DuplicatedSemanticId,
    LabelNotFound,
    Unexpected(anyhow::Error),
}

impl Into<String> for PostError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostError::NotFound => write!(f, "Post not found"),
            PostError::InvalidSemanticId => write!(
                f,
                "Semantic ID shouldn't be numeric and must conform to `^[0-9a-zA-Z_-]+$`"
            ),
            PostError::DuplicatedSemanticId => write!(f, "Semantic ID already exists"),
            PostError::LabelNotFound => write!(f, "One or more labels not found"),
            PostError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
