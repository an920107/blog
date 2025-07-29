use std::sync::Arc;

use crate::application::{error::auth_error::AuthError, gateway::auth_repository::AuthRepository};

pub trait GetAuthUrlUseCase: Send + Sync {
    fn execute(&self) -> Result<AuthUrl, AuthError>;
}

pub struct LoginUseCaseImpl {
    auth_repository: Arc<dyn AuthRepository>,
}

impl LoginUseCaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        Self { auth_repository }
    }
}

impl GetAuthUrlUseCase for LoginUseCaseImpl {
    fn execute(&self) -> Result<AuthUrl, AuthError> {
        self.auth_repository.get_auth_url()
    }
}

pub struct AuthUrl {
    pub url: String,
    pub state: String,
    pub nonce: String,
}
