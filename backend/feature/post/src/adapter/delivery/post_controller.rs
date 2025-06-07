use std::sync::Arc;

use async_trait::async_trait;

use crate::application::{
    error::post_error::PostError,
    use_case::{
        get_all_post_info_use_case::GetAllPostInfoUseCase,
        get_full_post_use_case::GetFullPostUseCase,
    },
};

use super::{post_info_response_dto::PostInfoResponseDto, post_response_dto::PostResponseDto};

#[async_trait]
pub trait PostController: Send + Sync {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
    ) -> Result<Vec<PostInfoResponseDto>, PostError>;

    async fn get_full_post(&self, id: i32) -> Result<PostResponseDto, PostError>;
}

pub struct PostControllerImpl {
    get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
    get_full_post_use_case: Arc<dyn GetFullPostUseCase>,
}

impl PostControllerImpl {
    pub fn new(
        get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
        get_full_post_use_case: Arc<dyn GetFullPostUseCase>,
    ) -> Self {
        Self {
            get_all_post_info_use_case,
            get_full_post_use_case,
        }
    }
}

#[async_trait]
impl PostController for PostControllerImpl {
    async fn get_all_post_info(
        &self,
        is_published_only: bool,
    ) -> Result<Vec<PostInfoResponseDto>, PostError> {
        let result = self.get_all_post_info_use_case.execute(is_published_only).await;

        result.map(|post_info_list| {
            let post_info_response_dto_list: Vec<PostInfoResponseDto> = post_info_list
                .into_iter()
                .map(|post_info| PostInfoResponseDto::from(post_info))
                .collect();

            post_info_response_dto_list
        })
    }

    async fn get_full_post(&self, id: i32) -> Result<PostResponseDto, PostError> {
        let result = self.get_full_post_use_case.execute(id).await;

        result.map(PostResponseDto::from)
    }
}
