use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::{post_db_mapper::PostMapper, post_info_db_mapper::PostInfoMapper},
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::{post::Post, post_info::PostInfo},
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
    ) -> Result<Vec<PostInfo>, PostError> {
        self.post_db_service
            .get_all_post_info(is_published_only, label_id)
            .await
            .map(|mappers| {
                mappers
                    .into_iter()
                    .map(|mapper| mapper.into_entity())
                    .collect::<Vec<PostInfo>>()
            })
    }

    async fn get_post_by_id(&self, id: i32) -> Result<Post, PostError> {
        self.post_db_service
            .get_post_by_id(id)
            .await
            .map(|mapper| mapper.into_entity())
    }

    async fn create_post(&self, post: Post, label_ids: &[i32]) -> Result<i32, PostError> {
        let info_mapper = PostInfoMapper {
            id: post.info.id,
            semantic_id: post.info.semantic_id,
            title: post.info.title,
            description: post.info.description,
            preview_image_url: post.info.preview_image_url,
            labels: Vec::new(),
            published_time: post.info.published_time.map(|dt| dt.naive_utc()),
        };

        let post_mapper = PostMapper {
            id: post.id,
            info: info_mapper,
            content: post.content,
        };

        self.post_db_service
            .create_post(post_mapper, label_ids)
            .await
    }

    async fn update_post(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError> {
        let info_mapper = PostInfoMapper {
            id: post.info.id,
            semantic_id: post.info.semantic_id,
            title: post.info.title,
            description: post.info.description,
            preview_image_url: post.info.preview_image_url,
            labels: Vec::new(),
            published_time: post.info.published_time.map(|dt| dt.naive_utc()),
        };

        let post_mapper = PostMapper {
            id: post.id,
            info: info_mapper,
            content: post.content,
        };

        self.post_db_service
            .update_post(post_mapper, label_ids)
            .await
    }

    async fn get_id_by_semantic_id(&self, semantic_id: &str) -> Result<i32, PostError> {
        self.post_db_service
            .get_id_by_semantic_id(semantic_id)
            .await
    }
}
