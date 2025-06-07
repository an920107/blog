use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::{
    adapter::gateway::post_db_service::PostDbService,
    application::error::post_error::PostError,
    domain::entity::{label::Label, post::Post, post_info::PostInfo},
};

use super::{
    post_info_with_label_record::PostInfoWithLabelRecord,
    post_with_label_record::PostWithLabelRecord,
};

pub struct PostDbServiceImpl {
    db_pool: Arc<Pool<Postgres>>,
}

impl PostDbServiceImpl {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl PostDbService for PostDbServiceImpl {
    async fn get_all_post_info(&self, is_published_only: bool) -> Result<Vec<PostInfo>, PostError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
                SELECT
                    p.id AS post_id,
                    p.title,
                    p.description,
                    p.preview_image_url,
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
                    p.deleted_time IS NULL
            "#,
        );

        if is_published_only {
            query_builder.push(r#" AND p.published_time IS NOT NULL"#);
        }

        query_builder.push(r#" ORDER BY p.id"#);

        let records = query_builder
            .build_query_as::<PostInfoWithLabelRecord>()
            .fetch_all(&*self.db_pool)
            .await
            .map_err(|err| PostError::DatabaseError(err.to_string()))?;

        let mut post_info_map = HashMap::<i32, PostInfo>::new();

        for record in records {
            let post_info = post_info_map
                .entry(record.post_id)
                .or_insert_with(|| PostInfo {
                    id: record.post_id,
                    title: record.title,
                    description: record.description,
                    preview_image_url: record.preview_image_url,
                    labels: Vec::new(),
                    published_time: record
                        .published_time
                        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
                });

            if let (Some(label_id), Some(label_name), Some(label_color)) =
                (record.label_id, record.label_name, record.label_color)
            {
                post_info.labels.push(Label {
                    id: label_id,
                    name: label_name,
                    color: label_color as u32,
                });
            }
        }

        Ok(post_info_map.into_values().collect())
    }

    async fn get_full_post(&self, id: i32) -> Result<Post, PostError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
                SELECT
                    p.id AS post_id,
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
        query_builder.push(r#" ORDER BY p.id"#);

        let records = query_builder
            .build_query_as::<PostWithLabelRecord>()
            .fetch_all(&*self.db_pool)
            .await
            .map_err(|err| PostError::DatabaseError(err.to_string()))?;

        if records.is_empty() {
            return Err(PostError::NotFound);
        }

        let mut post_map = HashMap::<i32, Post>::new();

        for record in records {
            let post = post_map.entry(record.post_id).or_insert_with(|| Post {
                id: record.post_id,
                info: PostInfo {
                    id: record.post_id,
                    title: record.title,
                    description: record.description,
                    preview_image_url: record.preview_image_url,
                    labels: Vec::new(),
                    published_time: record
                        .published_time
                        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
                },
                content: record.content,
            });

            if let (Some(label_id), Some(label_name), Some(label_color)) =
                (record.label_id, record.label_name, record.label_color)
            {
                post.info.labels.push(Label {
                    id: label_id,
                    name: label_name,
                    color: label_color as u32,
                });
            }
        }

        let post = post_map.into_values().next();

        match post {
            Some(v) => Ok(v),
            None => Err(PostError::NotFound),
        }
    }
}
