use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post_info::PostInfo,
};

#[async_trait]
pub trait GetAllPostInfoUseCase: Send + Sync {
    async fn execute(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Result<Vec<PostInfo>, PostError>;
}

pub struct GetAllPostInfoUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
}

impl GetAllPostInfoUseCaseImpl {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }
}

#[async_trait]
impl GetAllPostInfoUseCase for GetAllPostInfoUseCaseImpl {
    async fn execute(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
        user_id: Option<i32>,
    ) -> Result<Vec<PostInfo>, PostError> {
        let has_logged_in = user_id.is_some();

        // | is_published_only | has_logged_in | result |
        // | ----------------- | ------------- | ------ |
        // | T                 | T             | T      |
        // | T                 | F             | T      |
        // | F                 | T             | F      |
        // | F                 | F             | T      |
        let is_published_only = is_published_only || !has_logged_in;

        self.post_repository
            .get_all_post_info(is_published_only, label_id)
            .await
    }
}
