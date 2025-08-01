use async_trait::async_trait;

use crate::{
    adapter::gateway::user_db_mapper::UserMapper, application::error::auth_error::AuthError,
};

#[async_trait]
pub trait UserDbService: Send + Sync {
    async fn get_user_by_id(&self, user_id: i32) -> Result<UserMapper, AuthError>;

    async fn get_user_by_source_id(
        &self,
        issuer: &str,
        source_id: &str,
    ) -> Result<UserMapper, AuthError>;

    async fn create_user(&self, user: UserMapper) -> Result<i32, AuthError>;
}
