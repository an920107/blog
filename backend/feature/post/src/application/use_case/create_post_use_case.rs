use std::sync::Arc;

use async_trait::async_trait;
use label::application::gateway::label_repository::LabelRepository;

use crate::{
    application::{
        gateway::{
            create_post_params::CreatePostParams, post_repository::PostRepository,
            search_service::SearchService,
        },
        service::label_relation_service::LabelRelationService,
    },
    domain::error::post_error::PostError,
};

#[async_trait]
pub trait CreatePostUseCase: Send + Sync {
    async fn execute(&self, post: CreatePostParams) -> Result<i32, PostError>;
}

pub struct CreatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    label_repository: Arc<dyn LabelRepository>,
    post_search_indexer: Arc<dyn SearchService>,
}

impl CreatePostUseCaseImpl {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        label_repository: Arc<dyn LabelRepository>,
        post_search_indexer: Arc<dyn SearchService>,
    ) -> Self {
        Self {
            post_repository,
            label_repository,
            post_search_indexer,
        }
    }
}

#[async_trait]
impl CreatePostUseCase for CreatePostUseCaseImpl {
    async fn execute(&self, post: CreatePostParams) -> Result<i32, PostError> {
        post.validate()?;

        LabelRelationService::validate_labels_exist(self.label_repository.clone(), &post.label_ids)
            .await?;

        let id = self.post_repository.create_post(post.clone()).await?;
        let post = self.post_repository.get_post_by_id(id).await?;

        self.post_search_indexer.index_post(post).await?;

        Ok(id)
    }
}
