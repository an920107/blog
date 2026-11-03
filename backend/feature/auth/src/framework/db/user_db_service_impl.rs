use async_trait::async_trait;
use shared::framework::error::DatabaseError;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::{gateway::dto::user_db_model::UserDbModel, port::user_db_service::UserDbService},
    domain::error::auth_error::AuthError,
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
    async fn get_user_by_id(&self, user_id: i32) -> Result<UserDbModel, AuthError> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, issuer, source_id, displayed_name, email
            FROM "users"
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| AuthError::Unexpected(DatabaseError(e).into()))?;

        match record {
            Some(record) => Ok(record.into_db_model()),
            None => Err(AuthError::UserNotFound),
        }
    }

    async fn get_user_by_source_id(
        &self,
        issuer: &str,
        source_id: &str,
    ) -> Result<UserDbModel, AuthError> {
        let record = sqlx::query_as!(
            UserRecord,
            r#"
            SELECT id, issuer, source_id, displayed_name, email
            FROM "users"
            WHERE issuer = $1 AND source_id = $2
            "#,
            issuer,
            source_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| AuthError::Unexpected(DatabaseError(e).into()))?;

        match record {
            Some(record) => Ok(record.into_db_model()),
            None => Err(AuthError::UserNotFound),
        }
    }

    async fn create_user(&self, user: UserDbModel) -> Result<i32, AuthError> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO "users" (issuer, source_id, displayed_name, email)
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
        .map_err(|e| AuthError::Unexpected(DatabaseError(e).into()))?;

        Ok(id)
    }
}
