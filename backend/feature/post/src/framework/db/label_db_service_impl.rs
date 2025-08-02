use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::gateway::{label_db_mapper::LabelMapper, label_db_service::LabelDbService},
    application::error::post_error::PostError,
    framework::db::label_record::LabelRecord,
};

pub struct LabelDbServiceImpl {
    db_pool: Pool<Postgres>,
}

impl LabelDbServiceImpl {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl LabelDbService for LabelDbServiceImpl {
    async fn create_label(&self, label: LabelMapper) -> Result<i32, PostError> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO label (name, color)
            VALUES ($1, $2)
            RETURNING id
            "#,
            label.name,
            label.color.value as i64
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|err| PostError::DatabaseError(err.to_string()))?;

        Ok(id)
    }

    async fn update_label(&self, label: LabelMapper) -> Result<(), PostError> {
        let affected_rows = sqlx::query!(
            r#"
            UPDATE label
            SET name = $1, color = $2
            WHERE id = $3 AND deleted_time IS NULL
            "#,
            label.name,
            label.color.value as i64,
            label.id
        )
        .execute(&self.db_pool)
        .await
        .map_err(|err| PostError::DatabaseError(err.to_string()))?
        .rows_affected();

        if affected_rows == 0 {
            return Err(PostError::NotFound);
        }

        Ok(())
    }

    async fn get_label_by_id(&self, id: i32) -> Result<LabelMapper, PostError> {
        let record = sqlx::query_as!(
            LabelRecord,
            r#"
            SELECT id, name, color
            FROM label
            WHERE id = $1 AND deleted_time IS NULL
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|err| PostError::DatabaseError(err.to_string()))?;

        match record {
            Some(record) => Ok(record.into_mapper()),
            None => Err(PostError::NotFound),
        }
    }

    async fn get_all_labels(&self) -> Result<Vec<LabelMapper>, PostError> {
        let records = sqlx::query_as!(
            LabelRecord,
            r#"
            SELECT id, name, color
            FROM label
            WHERE deleted_time IS NULL
            ORDER BY id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|err| PostError::DatabaseError(err.to_string()))?;

        let mappers = records
            .into_iter()
            .map(|record| record.into_mapper())
            .collect();
        Ok(mappers)
    }
}
