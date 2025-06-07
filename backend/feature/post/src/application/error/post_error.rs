#[derive(Debug, PartialEq)]
pub enum PostError {
    DatabaseError(String),
    NotFound,
}
