use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use image::{
    application::port::image_reference_checker::ImageReferenceChecker,
    domain::error::image_error::ImageError,
};

use crate::{
    adapter::{
        gateway::dto::{
            post_info_db_model::PostInfoDbModel,
            post_write_db_model::{CreatePostDbModel, UpdatePostDbModel},
        },
        port::post_db_service::PostDbService,
    },
    application::port::{
        create_post_params::CreatePostParams, post_repository::PostRepository,
        update_post_params::UpdatePostParams,
    },
    domain::{
        entity::{post::Post, post_info::PostInfo},
        error::post_error::PostError,
    },
};

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
    ) -> Result<Vec<PostInfo>, PostError> {
        self.post_db_service
            .get_all_post_info(is_published_only, label_id)
            .await
            .map(|db_models: Vec<PostInfoDbModel>| {
                db_models
                    .into_iter()
                    .map(Into::into)
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
        let db_model = CreatePostDbModel {
            semantic_id: post.semantic_id,
            title: post.title,
            description: post.description,
            preview_image_url: post.preview_image_url,
            content: post.content,
            published_time: post.published_time.map(|dt| dt.naive_utc()),
            label_ids: post.label_ids,
            image_ids: post.image_ids,
        };

        self.post_db_service.create_post(db_model).await
    }

    async fn update_post(&self, id: i32, post: UpdatePostParams) -> Result<(), PostError> {
        let db_model = UpdatePostDbModel {
            id,
            title: post.title,
            description: post.description,
            preview_image_url: post.preview_image_url,
            content: post.content,
            published_time: post.published_time.map(|dt| dt.naive_utc()),
            label_ids: post.label_ids,
            image_ids: post.image_ids,
        };

        self.post_db_service.update_post(db_model).await
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
