use async_trait::async_trait;
use shared::framework::error::DatabaseError;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::{gateway::dto::label_db_model::LabelDbModel, port::label_db_service::LabelDbService},
    domain::error::label_error::LabelError,
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
    async fn create_label(&self, label: LabelDbModel) -> Result<i32, LabelError> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO labels (name, color)
            VALUES ($1, $2)
            RETURNING id
            "#,
            label.name,
            label.color.value as i64
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e
                && db_err.constraint() == Some("idx_labels_name")
            {
                return LabelError::DuplicatedLabelName;
            }
            LabelError::Unexpected(DatabaseError(e).into())
        })?;

        Ok(id)
    }

    async fn update_label(&self, label: LabelDbModel) -> Result<(), LabelError> {
        let affected_rows = sqlx::query!(
            r#"
            UPDATE labels
            SET name = $1, color = $2
            WHERE id = $3 AND deleted_time IS NULL
            "#,
            label.name,
            label.color.value as i64,
            label.id
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e
                && db_err.constraint() == Some("idx_labels_name")
            {
                return LabelError::DuplicatedLabelName;
            }
            LabelError::Unexpected(DatabaseError(e).into())
        })?
        .rows_affected();

        if affected_rows == 0 {
            return Err(LabelError::NotFound);
        }

        Ok(())
    }

    async fn get_label_by_id(&self, id: i32) -> Result<LabelDbModel, LabelError> {
        let record = sqlx::query_as!(
            LabelRecord,
            r#"
            SELECT id, name, color
            FROM labels
            WHERE id = $1 AND deleted_time IS NULL
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| LabelError::Unexpected(DatabaseError(e).into()))?;

        match record {
            Some(record) => Ok(record.into_db_model()),
            None => Err(LabelError::NotFound),
        }
    }

    async fn get_all_labels(&self) -> Result<Vec<LabelDbModel>, LabelError> {
        let records = sqlx::query_as!(
            LabelRecord,
            r#"
            SELECT id, name, color
            FROM labels
            WHERE deleted_time IS NULL
            ORDER BY id
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| LabelError::Unexpected(DatabaseError(e).into()))?;

        let db_models = records
            .into_iter()
            .map(|record| record.into_db_model())
            .collect();
        Ok(db_models)
    }
}
