use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::auth_error::AuthError, gateway::auth_repository::AuthRepository},
    domain::entity::user::User,
};

#[async_trait]
pub trait GetUserUseCase: Send + Sync {
    async fn execute(&self, user_id: i32) -> Result<User, AuthError>;
}

pub struct GetUserUseCaseImpl {
    auth_repository: Arc<dyn AuthRepository>,
}

impl GetUserUseCaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        Self { auth_repository }
    }
}

#[async_trait]
impl GetUserUseCase for GetUserUseCaseImpl {
    async fn execute(&self, user_id: i32) -> Result<User, AuthError> {
        self.auth_repository.get_user_by_id(user_id).await
    }
}
