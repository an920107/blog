use std::sync::Arc;

use crate::{
    application::gateway::auth_repository::AuthRepository, domain::error::auth_error::AuthError,
};

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
