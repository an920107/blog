#[derive(Debug, PartialEq)]
pub enum ImageError {
    DatabaseError(String),
    StorageError(String),
    NotFound,
    UnsupportedMimeType,
}
