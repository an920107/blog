use async_trait::async_trait;
use common::framework::error::DatabaseError;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::gateway::{image_db_mapper::ImageDbMapper, image_db_service::ImageDbService},
    domain::error::image_error::ImageError,
    framework::db::image_record::ImageRecord,
};

pub struct ImageDbServiceImpl {
    db_pool: Pool<Postgres>,
}

impl ImageDbServiceImpl {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ImageDbService for ImageDbServiceImpl {
    async fn create_image_info(&self, image: ImageDbMapper) -> Result<i32, ImageError> {
        let mime_type = image.mime_type.clone();
        let id = sqlx::query_scalar!(
            r#"
                INSERT INTO image (mime_type)
                VALUES ($1)
                RETURNING id
            "#,
            mime_type
        )
        .fetch_one(&self.db_pool)
        .await;

        match id {
            Ok(id) => Ok(id),
            Err(e) => Err(ImageError::Unexpected(DatabaseError(e).into())),
        }
    }

    async fn get_image_info_by_id(&self, id: i32) -> Result<ImageDbMapper, ImageError> {
        let image_record = sqlx::query_as!(
            ImageRecord,
            r#"
                SELECT id, mime_type
                FROM image
                WHERE id = $1 AND deleted_time IS NULL
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await;

        match image_record {
            Ok(record) => match record {
                Some(record) => Ok(ImageDbMapper {
                    id: record.id,
                    mime_type: record.mime_type,
                }),
                None => Err(ImageError::NotFound),
            },
            Err(e) => Err(ImageError::Unexpected(DatabaseError(e).into())),
        }
    }
}
