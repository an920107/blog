use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use image::application::gateway::image_repository::ImageRepository;

use crate::{
    application::{
        gateway::{post_repository::PostRepository, search_service::SearchService},
        service::preview_image_metadata_service::PreviewImageMetadataService,
    },
    domain::{entity::post_info::PostInfo, error::post_error::PostError},
};

#[async_trait]
pub trait GetAllPostInfoUseCase: Send + Sync {
    async fn execute(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
        keyword: Option<String>,
        user_id: Option<i32>,
    ) -> Result<Vec<PostInfo>, PostError>;
}

pub struct GetAllPostInfoUseCaseImpl {
    post_repository: Arc<dyn PostRepository>,
    search_service: Arc<dyn SearchService>,
    image_repository: Arc<dyn ImageRepository>,
}

impl GetAllPostInfoUseCaseImpl {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        search_service: Arc<dyn SearchService>,
        image_repository: Arc<dyn ImageRepository>,
    ) -> Self {
        Self {
            post_repository,
            search_service,
            image_repository,
        }
    }
}

#[async_trait]
impl GetAllPostInfoUseCase for GetAllPostInfoUseCaseImpl {
    async fn execute(
        &self,
        is_published_only: bool,
        label_id: Option<i32>,
        keyword: Option<String>,
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

        let mut posts = self
            .post_repository
            .get_all_post_info(is_published_only, label_id)
            .await?;

        PreviewImageMetadataService::attach(self.image_repository.clone(), &mut posts).await?;

        if let Some(keyword) = keyword {
            let post_map: HashMap<i32, PostInfo> =
                HashMap::from_iter(posts.into_iter().map(|post_info| (post_info.id, post_info)));

            let search_results: Vec<i32> = self
                .search_service
                .search_posts(&keyword, &Some(post_map.keys().copied().collect()))
                .await?;

            Ok(search_results
                .into_iter()
                .filter_map(|post_id| post_map.get(&post_id).cloned())
                .collect())
        } else {
            Ok(posts)
        }
    }
}
