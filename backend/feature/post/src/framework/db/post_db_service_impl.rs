use std::collections::HashMap;

use async_trait::async_trait;
use label::adapter::gateway::{
    dto::color_db_model::ColorDbModel, dto::label_db_model::LabelDbModel,
};
use shared::framework::error::DatabaseError;
use sqlx::{Pool, Postgres};

use crate::{
    adapter::{
        gateway::dto::{
            post_db_model::PostDbModel,
            post_info_db_model::PostInfoDbModel,
            post_write_db_model::{CreatePostDbModel, UpdatePostDbModel},
        },
        port::post_db_service::PostDbService,
    },
    domain::error::post_error::PostError,
};

use super::{
    post_info_with_label_record::PostInfoWithLabelRecord,
    post_with_label_record::PostWithLabelRecord,
};

pub struct PostDbServiceImpl {
    db_pool: Pool<Postgres>,
}

impl PostDbServiceImpl {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl PostDbService for PostDbServiceImpl {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
    ) -> Result<Vec<PostInfoDbModel>, PostError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
                SELECT
                    p.id AS post_id,
                    p.semantic_id,
                    p.title,
                    p.description,
                    p.preview_image_url,
                    p.published_time,
                    p.updated_time,
                    l.id AS label_id,
                    l.name AS label_name,
                    l.color AS label_color
                FROM
                    posts p
                LEFT JOIN
                    post_labels pl ON p.id = pl.post_id
                LEFT JOIN
                    labels l ON pl.label_id = l.id AND l.deleted_time IS NULL
                WHERE
                    p.deleted_time IS NULL
            "#,
        );

        if is_published_only {
            query_builder.push(r#" AND p.published_time IS NOT NULL"#);
        }

        if let Some(label_id) = label_id {
            // Only include posts that have the given label
            query_builder.push(r#" AND EXISTS (SELECT 1 FROM post_labels pl2 WHERE pl2.post_id = p.id AND pl2.label_id = "#);
            query_builder.push_bind(label_id);
            query_builder.push(r#")"#);
        }

        query_builder.push(
            r#" ORDER BY p.published_time DESC NULLS FIRST, p.updated_time DESC, pl."order""#,
        );

        let records = query_builder
            .build_query_as::<PostInfoWithLabelRecord>()
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let mut post_info_db_models_map = HashMap::<i32, PostInfoDbModel>::new();

        for record in &records {
            let post_info = post_info_db_models_map
                .entry(record.post_id)
                .or_insert_with(|| PostInfoDbModel {
                    id: record.post_id,
                    semantic_id: record.semantic_id.clone(),
                    title: record.title.clone(),
                    description: record.description.clone(),
                    preview_image_url: record.preview_image_url.clone(),
                    labels: Vec::new(),
                    published_time: record.published_time,
                    updated_time: record.updated_time,
                });

            if let (Some(label_id), Some(label_name), Some(label_color)) = (
                record.label_id,
                record.label_name.clone(),
                record.label_color,
            ) {
                post_info.labels.push(LabelDbModel {
                    id: label_id,
                    name: label_name,
                    color: ColorDbModel {
                        value: label_color as u32,
                    },
                });
            }
        }

        let mut ordered_posts = Vec::new();
        for record in &records {
            if let Some(post_info) = post_info_db_models_map.remove(&record.post_id) {
                ordered_posts.push(post_info);
            }
        }

        Ok(ordered_posts)
    }

    async fn get_post_by_id(&self, id: i32) -> Result<PostDbModel, PostError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
                SELECT
                    p.id AS post_id,
                    p.semantic_id,
                    p.title,
                    p.description,
                    p.preview_image_url,
                    p.content,
                    p.published_time,
                    p.updated_time,
                    l.id AS label_id,
                    l.name AS label_name,
                    l.color AS label_color
                FROM
                    posts p
                LEFT JOIN
                    post_labels pl ON p.id = pl.post_id
                LEFT JOIN
                    labels l ON pl.label_id = l.id AND l.deleted_time IS NULL
                WHERE
                    p.deleted_time IS NULL AND p.id =
            "#,
        );

        query_builder.push_bind(id);
        query_builder.push(r#" ORDER BY p.id, pl."order""#);

        let records = query_builder
            .build_query_as::<PostWithLabelRecord>()
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        if records.is_empty() {
            return Err(PostError::NotFound);
        }

        let mut post_db_models_map = HashMap::<i32, PostDbModel>::new();

        for record in &records {
            let post = post_db_models_map
                .entry(record.post_id)
                .or_insert_with(|| PostDbModel {
                    id: record.post_id,
                    info: PostInfoDbModel {
                        semantic_id: record.semantic_id.clone(),
                        id: record.post_id,
                        title: record.title.clone(),
                        description: record.description.clone(),
                        preview_image_url: record.preview_image_url.clone(),
                        labels: Vec::new(),
                        published_time: record.published_time,
                        updated_time: record.updated_time,
                    },
                    content: record.content.clone(),
                });

            if let (Some(label_id), Some(label_name), Some(label_color)) = (
                record.label_id,
                record.label_name.clone(),
                record.label_color,
            ) {
                post.info.labels.push(LabelDbModel {
                    id: label_id,
                    name: label_name,
                    color: ColorDbModel {
                        value: label_color as u32,
                    },
                });
            }
        }

        let post = post_db_models_map.into_values().next();

        match post {
            Some(v) => Ok(v),
            None => Err(PostError::NotFound),
        }
    }

    async fn create_post(&self, post: CreatePostDbModel) -> Result<i32, PostError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let post_id = sqlx::query_scalar!(
            r#"
            INSERT INTO posts (
                semantic_id, title, description, preview_image_url, content, published_time
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            post.semantic_id,
            post.title,
            post.description,
            post.preview_image_url,
            post.content,
            post.published_time,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e
                && db_err.constraint() == Some("idx_posts_semantic_id")
            {
                return PostError::DuplicatedSemanticId;
            }
            PostError::Unexpected(DatabaseError(e).into())
        })?;

        for (order, &label_id) in post.label_ids.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO post_labels (
                    post_id, label_id, "order"
                ) VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING
                "#,
                post_id,
                label_id,
                order as i32,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;
        }

        for &image_id in &post.image_ids {
            sqlx::query!(
                r#"
                INSERT INTO post_images (
                    post_id, image_id
                ) VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                post_id,
                image_id,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(post_id)
    }

    async fn update_post(&self, post: UpdatePostDbModel) -> Result<(), PostError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let affected_rows = sqlx::query!(
            r#"
            UPDATE posts
            SET
                title = $1,
                description = $2,
                preview_image_url = $3,
                content = $4,
                published_time = $5
            WHERE id = $6
            "#,
            post.title,
            post.description,
            post.preview_image_url,
            post.content,
            post.published_time,
            post.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e
                && db_err.constraint() == Some("idx_posts_semantic_id")
            {
                return PostError::DuplicatedSemanticId;
            }
            PostError::Unexpected(DatabaseError(e).into())
        })?
        .rows_affected();

        if affected_rows == 0 {
            return Err(PostError::NotFound);
        }

        sqlx::query!(
            r#"
            DELETE FROM post_labels
            WHERE post_id = $1
            "#,
            post.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        for (order, &label_id) in post.label_ids.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO post_labels (
                    post_id, label_id, "order"
                ) VALUES ($1, $2, $3)
                ON CONFLICT DO NOTHING
                "#,
                post.id,
                label_id,
                order as i32,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;
        }

        sqlx::query!(
            r#"
            DELETE FROM post_images
            WHERE post_id = $1
            "#,
            post.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        for &image_id in &post.image_ids {
            sqlx::query!(
                r#"
                INSERT INTO post_images (
                    post_id, image_id
                ) VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                post.id,
                image_id,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(())
    }

    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError> {
        let id = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM posts
            WHERE semantic_id = $1 AND deleted_time IS NULL
            "#,
            semantic_id,
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        match id {
            Some(id) => Ok(id),
            None => Err(PostError::NotFound),
        }
    }

    async fn count_by_image_id(&self, image_id: i32) -> Result<i64, PostError> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT
                count(*)
            FROM
                post_images
            WHERE
                image_id = $1
            "#,
            image_id,
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(count.unwrap_or(0))
    }

    async fn count_by_image_ids(&self, image_ids: &[i32]) -> Result<HashMap<i32, i64>, PostError> {
        if image_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let records = sqlx::query_as::<_, (i32, i64)>(
            r#"
            SELECT
                image_id,
                count(*)::BIGINT AS reference_count
            FROM
                post_images
            WHERE
                image_id = ANY($1)
            GROUP BY
                image_id
            "#,
        )
        .bind(image_ids)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(records.into_iter().collect())
    }
}
