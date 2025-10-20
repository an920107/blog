use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::{
        color_db_mapper::ColorMapper, label_db_mapper::LabelMapper,
        label_db_service::LabelDbService,
    },
    application::gateway::{
        create_or_update_label_params::CreateOrUpdateLabelParams, label_repository::LabelRepository,
    },
    domain::{entity::label::Label, error::label_error::LabelError},
};

pub struct LabelRepositoryImpl {
    label_db_service: Arc<dyn LabelDbService>,
}

impl LabelRepositoryImpl {
    pub fn new(label_db_service: Arc<dyn LabelDbService>) -> Self {
        Self { label_db_service }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryImpl {
    async fn create_label(&self, label: CreateOrUpdateLabelParams) -> Result<i32, LabelError> {
        self.label_db_service
            .create_label(LabelMapper {
                id: -1,
                name: label.name,
                color: ColorMapper::from(label.color),
            })
            .await
    }

    async fn update_label(
        &self,
        id: i32,
        label: CreateOrUpdateLabelParams,
    ) -> Result<(), LabelError> {
        self.label_db_service
            .update_label(LabelMapper {
                id,
                name: label.name,
                color: ColorMapper::from(label.color),
            })
            .await
    }

    async fn get_label_by_id(&self, id: i32) -> Result<Label, LabelError> {
        self.label_db_service
            .get_label_by_id(id)
            .await
            .map(Into::into)
    }

    async fn get_all_labels(&self) -> Result<Vec<Label>, LabelError> {
        self.label_db_service
            .get_all_labels()
            .await
            .map(|mappers| mappers.into_iter().map(Into::into).collect())
    }
}
