use std::fmt::Display;

#[derive(Debug)]
pub enum ImageError {
    NotFound,
    UnsupportedMimeType(String),
    ReferencedImage,
    ReferenceCheckFailed(String),
    Unexpected(anyhow::Error),
}

impl From<ImageError> for String {
    fn from(val: ImageError) -> Self {
        format!("{}", val)
    }
}

impl Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::NotFound => write!(f, "Image not found"),
            ImageError::UnsupportedMimeType(mime) => write!(f, "Unsupported MIME type: {}", mime),
            ImageError::ReferencedImage => write!(
                f,
                "Image is referenced by one or more posts and cannot be deleted"
            ),
            ImageError::ReferenceCheckFailed(e) => write!(f, "Reference check failed: {}", e),
            ImageError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
