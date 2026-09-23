use std::fmt::Display;

#[derive(Debug)]
pub enum SearchError {
    Unexpected(anyhow::Error),
}

impl From<SearchError> for String {
    fn from(val: SearchError) -> Self {
        format!("{}", val)
    }
}

impl Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
