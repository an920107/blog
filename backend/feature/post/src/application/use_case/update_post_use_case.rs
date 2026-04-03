use std::sync::Arc;

use async_trait::async_trait;
use label::application::gateway::label_repository::LabelRepository;

use crate::{
    application::{
        gateway::{
            post_repository::PostRepository, search_service::SearchService,
            update_post_params::UpdatePostParams,
        },
        service::label_relation_service::LabelRelationService,
    },
    domain::error::post_error::PostError,
};

#[async_trait]
pub trait UpdatePostUseCase: Send + Sync {
    async fn execute(&self, id: i32, post: UpdatePostParams) -> Result<(), PostError>;
}

pub struct UpdatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    label_repository: Arc<dyn LabelRepository>,
    post_search_indexer: Arc<dyn SearchService>,
}

impl UpdatePostUseCaseImpl {
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
impl UpdatePostUseCase for UpdatePostUseCaseImpl {
    async fn execute(&self, id: i32, post: UpdatePostParams) -> Result<(), PostError> {
        LabelRelationService::validate_labels_exist(self.label_repository.clone(), &post.label_ids)
            .await?;

        self.post_repository.update_post(id, post).await?;
        let post = self.post_repository.get_post_by_id(id).await?;

        self.post_search_indexer.index_post(post).await?;

        Ok(())
    }
}
