use std::collections::HashMap;

use async_trait::async_trait;
use common::framework::error::DatabaseError;
use label::adapter::gateway::{color_db_mapper::ColorMapper, label_db_mapper::LabelMapper};
use sqlx::{Pool, Postgres};

use crate::{
    adapter::gateway::{
        post_db_mapper::PostMapper, post_db_service::PostDbService,
        post_info_db_mapper::PostInfoMapper,
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
    ) -> Result<Vec<PostInfoMapper>, PostError> {
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
                    post p
                LEFT JOIN
                    post_label pl ON p.id = pl.post_id
                LEFT JOIN
                    label l ON pl.label_id = l.id AND l.deleted_time IS NULL
                WHERE
                    p.deleted_time IS NULL
            "#,
        );

        if is_published_only {
            query_builder.push(r#" AND p.published_time IS NOT NULL"#);
        }

        if let Some(label_id) = label_id {
            // Only include posts that have the given label
            query_builder.push(r#" AND EXISTS (SELECT 1 FROM post_label pl2 WHERE pl2.post_id = p.id AND pl2.label_id = "#);
            query_builder.push_bind(label_id);
            query_builder.push(r#")"#);
        }

        query_builder.push(r#" ORDER BY p.published_time DESC NULLS FIRST, p.updated_time DESC, pl."order""#);

        let records = query_builder
            .build_query_as::<PostInfoWithLabelRecord>()
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let mut post_info_mappers_map = HashMap::<i32, PostInfoMapper>::new();

        for record in &records {
            let post_info = post_info_mappers_map
                .entry(record.post_id)
                .or_insert_with(|| PostInfoMapper {
                    id: record.post_id,
                    semantic_id: record.semantic_id.clone(),
                    title: record.title.clone(),
                    description: record.description.clone(),
                    preview_image_url: record.preview_image_url.clone(),
                    labels: Vec::new(),
                    published_time: record.published_time,
                });

            if let (Some(label_id), Some(label_name), Some(label_color)) = (
                record.label_id,
                record.label_name.clone(),
                record.label_color,
            ) {
                post_info.labels.push(LabelMapper {
                    id: label_id,
                    name: label_name,
                    color: ColorMapper {
                        value: label_color as u32,
                    },
                });
            }
        }

        let mut ordered_posts = Vec::new();
        for record in &records {
            if let Some(post_info) = post_info_mappers_map.remove(&record.post_id) {
                ordered_posts.push(post_info);
            }
        }

        Ok(ordered_posts)
    }

    async fn get_post_by_id(&self, id: i32) -> Result<PostMapper, PostError> {
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
                    l.id AS label_id,
                    l.name AS label_name,
                    l.color AS label_color
                FROM
                    post p
                LEFT JOIN
                    post_label pl ON p.id = pl.post_id
                LEFT JOIN
                    label l ON pl.label_id = l.id AND l.deleted_time IS NULL
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

        let mut post_mappers_map = HashMap::<i32, PostMapper>::new();

        for record in &records {
            let post = post_mappers_map
                .entry(record.post_id)
                .or_insert_with(|| PostMapper {
                    id: record.post_id,
                    info: PostInfoMapper {
                        semantic_id: record.semantic_id.clone(),
                        id: record.post_id,
                        title: record.title.clone(),
                        description: record.description.clone(),
                        preview_image_url: record.preview_image_url.clone(),
                        labels: Vec::new(),
                        published_time: record.published_time,
                    },
                    content: record.content.clone(),
                });

            if let (Some(label_id), Some(label_name), Some(label_color)) = (
                record.label_id,
                record.label_name.clone(),
                record.label_color,
            ) {
                post.info.labels.push(LabelMapper {
                    id: label_id,
                    name: label_name,
                    color: ColorMapper {
                        value: label_color as u32,
                    },
                });
            }
        }

        let post = post_mappers_map.into_values().next();

        match post {
            Some(v) => Ok(v),
            None => Err(PostError::NotFound),
        }
    }

    async fn create_post(&self, post: PostMapper, label_ids: &[i32]) -> Result<i32, PostError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let post_id = sqlx::query_scalar!(
            r#"
            INSERT INTO post (
                semantic_id, title, description, preview_image_url, content, published_time
            ) VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            post.info.semantic_id,
            post.info.title,
            post.info.description,
            post.info.preview_image_url,
            post.content,
            post.info.published_time,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("idx_post_semantic_id") {
                    return PostError::DuplicatedSemanticId;
                }
            }
            PostError::Unexpected(DatabaseError(e).into())
        })?;

        for (order, &label_id) in label_ids.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO post_label (
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

        tx.commit()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(post_id)
    }

    async fn update_post(&self, post: PostMapper, label_ids: &[i32]) -> Result<(), PostError> {
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        let affected_rows = sqlx::query!(
            r#"
            UPDATE post
            SET 
                title = $1, 
                description = $2, 
                preview_image_url = $3, 
                content = $4, 
                published_time = $5
            WHERE id = $6
            "#,
            post.info.title,
            post.info.description,
            post.info.preview_image_url,
            post.content,
            post.info.published_time,
            post.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("idx_post_semantic_id") {
                    return PostError::DuplicatedSemanticId;
                }
            }
            PostError::Unexpected(DatabaseError(e).into())
        })?
        .rows_affected();

        if affected_rows == 0 {
            return Err(PostError::NotFound);
        }

        sqlx::query!(
            r#"
            DELETE FROM post_label
            WHERE post_id = $1
            "#,
            post.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        for (order, &label_id) in label_ids.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO post_label (
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

        tx.commit()
            .await
            .map_err(|e| PostError::Unexpected(DatabaseError(e).into()))?;

        Ok(())
    }

    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError> {
        let id = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM post
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
}
