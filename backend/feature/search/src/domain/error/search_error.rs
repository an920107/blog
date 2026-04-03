use std::fmt::Display;

#[derive(Debug)]
pub enum SearchError {
    Unexpected(anyhow::Error),
}

impl Into<String> for SearchError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
