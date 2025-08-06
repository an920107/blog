use std::fmt::Display;

#[derive(Debug)]
pub enum ImageError {
    NotFound,
    UnsupportedMimeType(String),
    Unexpected(anyhow::Error),
}

impl Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::NotFound => write!(f, "Image not found"),
            ImageError::UnsupportedMimeType(mime) => write!(f, "Unsupported MIME type: {}", mime),
            ImageError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
