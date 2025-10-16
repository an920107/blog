use std::sync::Arc;

use async_trait::async_trait;
use label::application::gateway::label_repository::LabelRepository;

use crate::{
    application::{error::post_error::PostError, gateway::post_repository::PostRepository},
    domain::entity::post::Post,
};

#[async_trait]
pub trait UpdatePostUseCase: Send + Sync {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError>;
}

pub struct UpdatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    label_repository: Arc<dyn LabelRepository>,
}

impl UpdatePostUseCaseImpl {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        label_repository: Arc<dyn LabelRepository>,
    ) -> Self {
        Self {
            post_repository,
            label_repository,
        }
    }
}

#[async_trait]
impl UpdatePostUseCase for UpdatePostUseCaseImpl {
    async fn execute(&self, post: Post, label_ids: &[i32]) -> Result<(), PostError> {
        post.validate()?;

        // Check if all label IDs exist
        for &label_id in label_ids {
            if let Err(_) = self.label_repository.get_label_by_id(label_id).await {
                return Err(PostError::LabelNotFound);
            }
        }

        self.post_repository.update_post(post, label_ids).await
    }
}
