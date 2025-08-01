use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::gateway::{user_db_mapper::UserMapper, user_db_service::UserDbService},
    application::error::auth_error::AuthError,
    framework::db::user_record::UserRecord,
};

pub struct UserDbServiceImpl {
    db_pool: Pool<Postgres>,
}

impl UserDbServiceImpl {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserDbService for UserDbServiceImpl {
    async fn get_user_by_id(&self, user_id: i32) -> Result<UserMapper, AuthError> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, issuer, source_id, displayed_name, email
            FROM "user"
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        match record {
            Some(record) => Ok(record.into_mapper()),
            None => Err(AuthError::UserNotFound),
        }
    }

    async fn get_user_by_source_id(
        &self,
        issuer: &str,
        source_id: &str,
    ) -> Result<UserMapper, AuthError> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, issuer, source_id, displayed_name, email
            FROM "user"
            WHERE issuer = $1 AND source_id = $2
            "#,
            issuer,
            source_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        match record {
            Some(record) => Ok(record.into_mapper()),
            None => Err(AuthError::UserNotFound),
        }
    }

    async fn create_user(&self, user: UserMapper) -> Result<i32, AuthError> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO "user" (issuer, source_id, displayed_name, email)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            user.issuer,
            user.source_id,
            user.displayed_name,
            user.email
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(id)
    }
}
