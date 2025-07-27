use std::sync::Arc;

use async_trait::async_trait;

use crate::{
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
    async fn get_all_post_info(&self, is_published_only: bool) -> Result<Vec<PostInfo>, PostError> {
        self.post_db_service
            .get_all_post_info(is_published_only)
            .await
            .map(|mappers| {
                mappers
                    .into_iter()
                    .map(|mapper| mapper.into_entity())
                    .collect::<Vec<PostInfo>>()
            })
    }

    async fn get_full_post(&self, id: i32) -> Result<Post, PostError> {
        self.post_db_service
            .get_full_post(id)
            .await
            .map(|mapper| mapper.into_entity())
    }
}
