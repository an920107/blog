use crate::domain::error::post_error::PostError;
use label::application::gateway::label_repository::LabelRepository;
use std::sync::Arc;

pub struct LabelRelationService;

impl LabelRelationService {
    pub async fn validate_labels_exist(
        label_repository: Arc<dyn LabelRepository>,
        label_ids: &[i32],
    ) -> Result<(), PostError> {
        for &label_id in label_ids {
            label_repository
                .get_label_by_id(label_id)
                .await
                .map_err(|e| PostError::LabelError(e.into()))?;
        }

        Ok(())
    }
}
