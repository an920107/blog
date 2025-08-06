use std::fmt::Display;

#[derive(Debug)]
pub enum PostError {
    NotFound,
    Unexpected(anyhow::Error),
}

impl Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostError::NotFound => write!(f, "Post not found"),
            PostError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
