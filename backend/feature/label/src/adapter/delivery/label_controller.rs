use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::delivery::{
        create_label_request_dto::CreateLabelRequestDto, label_response_dto::LabelResponseDto,
        update_label_request_dto::UpdateLabelRequestDto,
    },
    application::{
        error::label_error::LabelError,
        use_case::{
            create_label_use_case::CreateLabelUseCase,
            get_all_labels_use_case::GetAllLabelsUseCase,
            get_label_by_id_use_case::GetLabelByIdUseCase,
            update_label_use_case::UpdateLabelUseCase,
        },
    },
};

#[async_trait]
pub trait LabelController: Send + Sync {
    async fn create_label(
        &self,
        label: CreateLabelRequestDto,
    ) -> Result<LabelResponseDto, LabelError>;

    async fn update_label(
        &self,
        id: i32,
        label: UpdateLabelRequestDto,
    ) -> Result<LabelResponseDto, LabelError>;

    async fn get_all_labels(&self) -> Result<Vec<LabelResponseDto>, LabelError>;

    async fn get_label_by_id(&self, id: i32) -> Result<LabelResponseDto, LabelError>;
}

pub struct LabelControllerImpl {
    create_label_use_case: Arc<dyn CreateLabelUseCase>,
    update_label_use_case: Arc<dyn UpdateLabelUseCase>,
    get_all_labels_use_case: Arc<dyn GetAllLabelsUseCase>,
    get_label_by_id_use_case: Arc<dyn GetLabelByIdUseCase>,
}

impl LabelControllerImpl {
    pub fn new(
        create_label_use_case: Arc<dyn CreateLabelUseCase>,
        update_label_use_case: Arc<dyn UpdateLabelUseCase>,
        get_all_labels_use_case: Arc<dyn GetAllLabelsUseCase>,
        get_label_by_id_use_case: Arc<dyn GetLabelByIdUseCase>,
    ) -> Self {
        Self {
            create_label_use_case,
            update_label_use_case,
            get_all_labels_use_case,
            get_label_by_id_use_case,
        }
    }
}

#[async_trait]
impl LabelController for LabelControllerImpl {
    async fn create_label(
        &self,
        label: CreateLabelRequestDto,
    ) -> Result<LabelResponseDto, LabelError> {
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
    ) -> Result<LabelResponseDto, LabelError> {
        let label_entity = label.into_entity(id);
        self.update_label_use_case
            .execute(label_entity.clone())
            .await?;

        Ok(LabelResponseDto::from(label_entity))
    }

    async fn get_all_labels(&self) -> Result<Vec<LabelResponseDto>, LabelError> {
        let result = self.get_all_labels_use_case.execute().await?;

        Ok(result
            .into_iter()
            .map(|label| LabelResponseDto::from(label))
            .collect())
    }

    async fn get_label_by_id(&self, id: i32) -> Result<LabelResponseDto, LabelError> {
        let label = self.get_label_by_id_use_case.execute(id).await?;

        Ok(LabelResponseDto::from(label))
    }
}
