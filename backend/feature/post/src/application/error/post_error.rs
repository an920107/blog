use std::fmt::Display;

#[derive(Debug)]
pub enum PostError {
    NotFound,
    Unauthorized,
    InvalidSemanticId,
    DuplicatedSemanticId,
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
            PostError::Unauthorized => write!(f, "Unauthorized access"),
            PostError::InvalidSemanticId => write!(
                f,
                "Semantic ID shouldn't be numeric and must conform to `^[0-9a-zA-Z_-]+$`"
            ),
            PostError::DuplicatedSemanticId => write!(f, "Semantic ID already exists"),
            PostError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
