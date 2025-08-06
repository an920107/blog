use std::fmt::Display;

#[derive(Debug)]
pub struct IOError(pub std::io::Error);

#[derive(Debug)]
pub struct DatabaseError(pub sqlx::Error);

impl std::error::Error for IOError {}
impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DatabaseError {}
impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
