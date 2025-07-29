#[derive(Debug, PartialEq)]
pub enum AuthError {
    OidcError(String),
    InvalidState,
    InvalidNonce,
    InvalidAuthCode,
    InvalidIdToken,
}
