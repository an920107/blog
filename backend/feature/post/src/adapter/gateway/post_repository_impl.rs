use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use image::{
    application::gateway::image_reference_checker::ImageReferenceChecker,
    domain::error::image_error::ImageError,
};

use crate::{
    adapter::gateway::{post_db_mapper::PostMapper, post_info_db_mapper::PostInfoMapper},
    application::gateway::{
        create_post_params::CreatePostParams, post_repository::PostRepository,
        update_post_params::UpdatePostParams,
    },
    domain::{
        entity::{post::Post, post_info::PostInfo},
        error::post_error::PostError,
    },
};

use super::post_db_service::PostDbService;

pub struct PostRepositoryImpl {
    post_db_service: Arc<dyn PostDbService>,
}

impl PostRepositoryImpl {
    pub fn new(post_db_service: Arc<dyn PostDbService>) -> Self {
        Self { post_db_service }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
        keyword: Option<String>,
    ) -> Result<Vec<PostInfo>, PostError> {
        self.post_db_service
            .get_all_post_info(is_published_only, label_id)
            .await
            .map(|mappers: Vec<PostInfoMapper>| {
                mappers
                    .into_iter()
                    .map(Into::into)
                    // TODO: For now, I'm using filters to filter only the titles;
                    // I'll switch to embeddings for searching later.
                    .filter(|post: &PostInfo| {
                        if let Some(keyword) = &keyword {
                            post.title.to_lowercase().contains(&keyword.to_lowercase())
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<PostInfo>>()
            })
    }

    async fn get_post_by_id(&self, id: i32) -> Result<Post, PostError> {
        self.post_db_service
            .get_post_by_id(id)
            .await
            .map(Into::into)
    }

    async fn create_post(&self, post: CreatePostParams) -> Result<i32, PostError> {
        let info_mapper = PostInfoMapper {
            id: -1,
            semantic_id: post.semantic_id,
            title: post.title,
            description: post.description,
            preview_image_url: post.preview_image_url,
            labels: Vec::new(),
            published_time: post.published_time.map(|dt| dt.naive_utc()),
        };

        let post_mapper = PostMapper {
            id: -1,
            info: info_mapper,
            content: post.content,
        };

        self.post_db_service
            .create_post(post_mapper, &post.label_ids, &post.image_ids)
            .await
    }

    async fn update_post(&self, id: i32, post: UpdatePostParams) -> Result<(), PostError> {
        let info_mapper = PostInfoMapper {
            id: id,
            semantic_id: String::new(),
            title: post.title,
            description: post.description,
            preview_image_url: post.preview_image_url,
            labels: Vec::new(),
            published_time: post.published_time.map(|dt| dt.naive_utc()),
        };

        let post_mapper = PostMapper {
            id: id,
            info: info_mapper,
            content: post.content,
        };

        self.post_db_service
            .update_post(post_mapper, &post.label_ids, &post.image_ids)
            .await
    }

    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError> {
        self.post_db_service
            .get_id_by_semantic_id(semantic_id)
            .await
    }

    async fn count_by_image_id(&self, image_id: i32) -> Result<i64, PostError> {
        self.post_db_service.count_by_image_id(image_id).await
    }
}

#[async_trait]
impl ImageReferenceChecker for PostRepositoryImpl {
    async fn get_reference_counts(
        &self,
        image_ids: &[i32],
    ) -> Result<HashMap<i32, i64>, ImageError> {
        self.post_db_service
            .count_by_image_ids(image_ids)
            .await
            .map_err(|e| ImageError::ReferenceCheckFailed(e.to_string()))
    }

    async fn is_image_referenced(&self, image_id: i32) -> Result<bool, ImageError> {
        let reference_counts = self.get_reference_counts(&[image_id]).await?;

        Ok(reference_counts.get(&image_id).copied().unwrap_or(0) > 0)
    }
}
