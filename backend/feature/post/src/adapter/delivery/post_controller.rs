use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        create_post_request_dto::CreatePostRequestDto, post_info_query_dto::PostQueryDto,
        update_post_request_dto::UpdatePostRequestDto,
    },
    application::use_case::{
        create_post_use_case::CreatePostUseCase, get_all_post_info_use_case::GetAllPostInfoUseCase,
        get_post_by_id_use_case::GetPostByIdUseCase,
        get_post_by_semantic_id_use_case::GetPostBySemanticIdUseCase,
        update_post_use_case::UpdatePostUseCase,
    },
    domain::error::post_error::PostError,
};

use super::{post_info_response_dto::PostInfoResponseDto, post_response_dto::PostResponseDto};

#[async_trait]
pub trait PostController: Send + Sync {
    async fn get_all_post_info(
        &self,
        query: PostQueryDto,
        user_id: Option<i32>,
    ) -> Result<Vec<PostInfoResponseDto>, PostError>;

    async fn get_post_by_id_or_semantic_id(
        &self,
        id_or_semantic_id: &str,
        user_id: Option<i32>,
    ) -> Result<PostResponseDto, PostError>;

    async fn create_post(
        &self,
        post: CreatePostRequestDto,
        user_id: i32,
    ) -> Result<PostResponseDto, PostError>;

    async fn update_post(
        &self,
        id: i32,
        post: UpdatePostRequestDto,
        user_id: i32,
    ) -> Result<PostResponseDto, PostError>;
}

pub struct PostControllerImpl {
    get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
    get_post_by_id_use_case: Arc<dyn GetPostByIdUseCase>,
    get_post_by_semantic_id_use_case: Arc<dyn GetPostBySemanticIdUseCase>,
    create_post_use_case: Arc<dyn CreatePostUseCase>,
    update_post_use_case: Arc<dyn UpdatePostUseCase>,
}

impl PostControllerImpl {
    pub fn new(
        get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
        get_post_by_id_use_case: Arc<dyn GetPostByIdUseCase>,
        get_post_by_semantic_id_use_case: Arc<dyn GetPostBySemanticIdUseCase>,
        create_post_use_case: Arc<dyn CreatePostUseCase>,
        update_post_use_case: Arc<dyn UpdatePostUseCase>,
    ) -> Self {
        Self {
            get_all_post_info_use_case,
            get_post_by_id_use_case,
            get_post_by_semantic_id_use_case,
            create_post_use_case,
            update_post_use_case,
        }
    }

    async fn get_post_by_id(
        &self,
        id: i32,
        user_id: Option<i32>,
    ) -> Result<PostResponseDto, PostError> {
        let result = self.get_post_by_id_use_case.execute(id, user_id).await;

        result.map(PostResponseDto::from)
    }

    async fn get_post_by_semantic_id(
        &self,
        semantic_id: &str,
        user_id: Option<i32>,
    ) -> Result<PostResponseDto, PostError> {
        let result = self
            .get_post_by_semantic_id_use_case
            .execute(semantic_id, user_id)
            .await;

        result.map(PostResponseDto::from)
    }
}

#[async_trait]
impl PostController for PostControllerImpl {
    async fn get_all_post_info(
        &self,
        query: PostQueryDto,
        user_id: Option<i32>,
    ) -> Result<Vec<PostInfoResponseDto>, PostError> {
        let result = self
            .get_all_post_info_use_case
            .execute(
                query.is_published_only.unwrap_or(true),
                query.label_id,
                user_id,
            )
            .await;

        result.map(|post_info_list| {
            let post_info_response_dto_list: Vec<PostInfoResponseDto> = post_info_list
                .into_iter()
                .map(|post_info| PostInfoResponseDto::from(post_info))
                .collect();

            post_info_response_dto_list
        })
    }

    async fn get_post_by_id_or_semantic_id(
        &self,
        id_or_semantic_id: &str,
        user_id: Option<i32>,
    ) -> Result<PostResponseDto, PostError> {
        if let Ok(id) = id_or_semantic_id.parse::<i32>() {
            self.get_post_by_id(id, user_id).await
        } else {
            let semantic_id = id_or_semantic_id;
            self.get_post_by_semantic_id(semantic_id, user_id).await
        }
    }

    async fn create_post(
        &self,
        post: CreatePostRequestDto,
        user_id: i32,
    ) -> Result<PostResponseDto, PostError> {
        let id = self.create_post_use_case.execute(post.into()).await?;

        self.get_post_by_id(id, Some(user_id)).await
    }

    async fn update_post(
        &self,
        id: i32,
        post: UpdatePostRequestDto,
        user_id: i32,
    ) -> Result<PostResponseDto, PostError> {
        self.update_post_use_case.execute(id, post.into()).await?;

        self.get_post_by_id(id, Some(user_id)).await
    }
}
