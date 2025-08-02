use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::gateway::{label_db_mapper::LabelMapper, label_db_service::LabelDbService},
    application::{error::post_error::PostError, gateway::label_repository::LabelRepository},
    domain::entity::label::Label,
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
    async fn create_label(&self, label: Label) -> Result<i32, PostError> {
        self.label_db_service
            .create_label(LabelMapper::from(label))
            .await
    }

    async fn update_label(&self, label: Label) -> Result<(), PostError> {
        self.label_db_service
            .update_label(LabelMapper::from(label))
            .await
    }

    async fn get_label_by_id(&self, id: i32) -> Result<Label, PostError> {
        self.label_db_service
            .get_label_by_id(id)
            .await
            .map(|mapper| mapper.into_entity())
    }

    async fn get_all_labels(&self) -> Result<Vec<Label>, PostError> {
        self.label_db_service.get_all_labels().await.map(|mappers| {
            mappers
                .into_iter()
                .map(|mapper| mapper.into_entity())
                .collect()
        })
    }
}
