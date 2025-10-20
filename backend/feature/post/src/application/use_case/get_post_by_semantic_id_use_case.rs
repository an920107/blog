use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{
        gateway::post_repository::PostRepository,
        use_case::get_post_by_id_use_case::GetPostByIdUseCase,
    },
    domain::{entity::post::Post, error::post_error::PostError},
};

#[async_trait]
pub trait GetPostBySemanticIdUseCase: Send + Sync {
    async fn execute(&self, semantic_id: &str, user_id: Option<i32>) -> Result<Post, PostError>;
}

pub struct GetPostBySemanticIdUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    get_post_by_id_use_case: Arc<dyn GetPostByIdUseCase>,
}

impl GetPostBySemanticIdUseCaseImpl {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        get_post_by_id_use_case: Arc<dyn GetPostByIdUseCase>,
    ) -> Self {
        Self {
            post_repository,
            get_post_by_id_use_case,
        }
    }
}

#[async_trait]
impl GetPostBySemanticIdUseCase for GetPostBySemanticIdUseCaseImpl {
    async fn execute(&self, semantic_id: &str, user_id: Option<i32>) -> Result<Post, PostError> {
        let id = self
            .post_repository
            .get_id_by_semantic_id(semantic_id)
            .await?;

        self.get_post_by_id_use_case.execute(id, user_id).await
    }
}
