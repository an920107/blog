use std::fmt::Display;

#[derive(Debug)]
pub enum AuthError {
    InvalidState,
    InvalidNonce,
    InvalidAuthCode,
    InvalidIdToken,
    UserNotFound,
    Unexpected(anyhow::Error),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidState => write!(f, "Invalid state"),
            AuthError::InvalidNonce => write!(f, "Invalid nonce"),
            AuthError::InvalidAuthCode => write!(f, "Invalid authentication code"),
            AuthError::InvalidIdToken => write!(f, "Invalid ID token"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::Unexpected(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}
