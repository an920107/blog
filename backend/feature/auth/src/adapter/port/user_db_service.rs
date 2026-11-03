use async_trait::async_trait;

use crate::{
    adapter::gateway::dto::user_db_model::UserDbModel, domain::error::auth_error::AuthError,
};

#[async_trait]
pub trait UserDbService: Send + Sync {
    async fn get_user_by_id(&self, user_id: i32) -> Result<UserDbModel, AuthError>;

    async fn get_user_by_source_id(
        &self,
        issuer: &str,
        source_id: &str,
    ) -> Result<UserDbModel, AuthError>;

    async fn create_user(&self, user: UserDbModel) -> Result<i32, AuthError>;
}
