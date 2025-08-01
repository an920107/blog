#[derive(Debug, PartialEq)]
pub enum AuthError {
    DatabaseError(String),
    OidcError(String),
    InvalidState,
    InvalidNonce,
    InvalidAuthCode,
    InvalidIdToken,
    UserNotFound,
}
