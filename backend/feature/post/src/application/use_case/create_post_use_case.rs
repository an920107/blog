use std::sync::Arc;

use async_trait::async_trait;
use label::application::gateway::label_repository::LabelRepository;

use crate::{
    application::gateway::{create_post_params::CreatePostParams, post_repository::PostRepository},
    domain::error::post_error::PostError,
};

#[async_trait]
pub trait CreatePostUseCase: Send + Sync {
    async fn execute(&self, post: CreatePostParams) -> Result<i32, PostError>;
}

pub struct CreatePostUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    label_repository: Arc<dyn LabelRepository>,
}

impl CreatePostUseCaseImpl {
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
impl CreatePostUseCase for CreatePostUseCaseImpl {
    async fn execute(&self, post: CreatePostParams) -> Result<i32, PostError> {
        post.validate()?;

        // Check if all label IDs exist
        for &label_id in post.label_ids.iter() {
            if let Err(_) = self.label_repository.get_label_by_id(label_id).await {
                return Err(PostError::LabelNotFound);
            }
        }

        self.post_repository.create_post(post).await
    }
}
