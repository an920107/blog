use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        create_label_request_dto::CreateLabelRequestDto,
        create_post_request_dto::CreatePostRequestDto, post_info_query_dto::PostQueryDto,
        update_label_request_dto::UpdateLabelRequestDto,
        update_post_request_dto::UpdatePostRequestDto,
    },
    application::{
        error::post_error::PostError,
        use_case::{
            create_label_use_case::CreateLabelUseCase, create_post_use_case::CreatePostUseCase,
            get_all_labels_use_case::GetAllLabelsUseCase,
            get_all_post_info_use_case::GetAllPostInfoUseCase,
            get_full_post_use_case::GetFullPostUseCase, update_label_use_case::UpdateLabelUseCase,
            update_post_use_case::UpdatePostUseCase,
        },
    },
};

use super::{
    label_response_dto::LabelResponseDto, post_info_response_dto::PostInfoResponseDto,
    post_response_dto::PostResponseDto,
};

#[async_trait]
pub trait PostController: Send + Sync {
    async fn get_all_post_info(
        &self,
        query: PostQueryDto,
    ) -> Result<Vec<PostInfoResponseDto>, PostError>;

    async fn get_post_by_id(&self, id: i32) -> Result<PostResponseDto, PostError>;

    async fn create_post(&self, post: CreatePostRequestDto) -> Result<PostResponseDto, PostError>;

    async fn update_post(
        &self,
        id: i32,
        post: UpdatePostRequestDto,
    ) -> Result<PostResponseDto, PostError>;

    async fn create_label(
        &self,
        label: CreateLabelRequestDto,
    ) -> Result<LabelResponseDto, PostError>;

    async fn update_label(
        &self,
        id: i32,
        label: UpdateLabelRequestDto,
    ) -> Result<LabelResponseDto, PostError>;

    async fn get_all_labels(&self) -> Result<Vec<LabelResponseDto>, PostError>;
}

pub struct PostControllerImpl {
    get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
    get_full_post_use_case: Arc<dyn GetFullPostUseCase>,
    create_post_use_case: Arc<dyn CreatePostUseCase>,
    update_post_use_case: Arc<dyn UpdatePostUseCase>,
    create_label_use_case: Arc<dyn CreateLabelUseCase>,
    update_label_use_case: Arc<dyn UpdateLabelUseCase>,
    get_all_labels_use_case: Arc<dyn GetAllLabelsUseCase>,
}

impl PostControllerImpl {
    pub fn new(
        get_all_post_info_use_case: Arc<dyn GetAllPostInfoUseCase>,
        get_full_post_use_case: Arc<dyn GetFullPostUseCase>,
        create_post_use_case: Arc<dyn CreatePostUseCase>,
        update_post_use_case: Arc<dyn UpdatePostUseCase>,
        create_label_use_case: Arc<dyn CreateLabelUseCase>,
        update_label_use_case: Arc<dyn UpdateLabelUseCase>,
        get_all_labels_use_case: Arc<dyn GetAllLabelsUseCase>,
    ) -> Self {
        Self {
            get_all_post_info_use_case,
            get_full_post_use_case,
            create_post_use_case,
            update_post_use_case,
            create_label_use_case,
            update_label_use_case,
            get_all_labels_use_case,
        }
    }
}

#[async_trait]
impl PostController for PostControllerImpl {
    async fn get_all_post_info(
        &self,
        query: PostQueryDto,
    ) -> Result<Vec<PostInfoResponseDto>, PostError> {
        let result = self
            .get_all_post_info_use_case
            .execute(query.is_published_only.unwrap_or(true))
            .await;

        result.map(|post_info_list| {
            let post_info_response_dto_list: Vec<PostInfoResponseDto> = post_info_list
                .into_iter()
                .map(|post_info| PostInfoResponseDto::from(post_info))
                .collect();

            post_info_response_dto_list
        })
    }

    async fn get_post_by_id(&self, id: i32) -> Result<PostResponseDto, PostError> {
        let result = self.get_full_post_use_case.execute(id).await;

        result.map(PostResponseDto::from)
    }

    async fn create_label(
        &self,
        label: CreateLabelRequestDto,
    ) -> Result<LabelResponseDto, PostError> {
        let mut label_entity = label.into_entity();
        let id = self
            .create_label_use_case
            .execute(label_entity.clone())
            .await?;

        label_entity.id = id;
        Ok(LabelResponseDto::from(label_entity))
    }

    async fn update_label(
        &self,
        id: i32,
        label: UpdateLabelRequestDto,
    ) -> Result<LabelResponseDto, PostError> {
        let label_entity = label.into_entity(id);
        self.update_label_use_case
            .execute(label_entity.clone())
            .await?;

        Ok(LabelResponseDto::from(label_entity))
    }

    async fn get_all_labels(&self) -> Result<Vec<LabelResponseDto>, PostError> {
        let result = self.get_all_labels_use_case.execute().await;

        result.map(|labels| {
            labels
                .into_iter()
                .map(|label| LabelResponseDto::from(label))
                .collect()
        })
    }

    async fn create_post(&self, post: CreatePostRequestDto) -> Result<PostResponseDto, PostError> {
        let label_ids = post.label_ids.clone();
        let post_entity = post.into_entity();

        let id = self
            .create_post_use_case
            .execute(post_entity, &label_ids)
            .await?;

        self.get_post_by_id(id).await
    }

    async fn update_post(
        &self,
        id: i32,
        post: UpdatePostRequestDto,
    ) -> Result<PostResponseDto, PostError> {
        let label_ids = post.label_ids.clone();
        let post_entity = post.into_entity(id);

        self.update_post_use_case
            .execute(post_entity, &label_ids)
            .await?;

        self.get_post_by_id(id).await
    }
}
