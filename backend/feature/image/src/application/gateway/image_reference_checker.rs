use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::error::image_error::ImageError;

#[async_trait]
pub trait ImageReferenceChecker: Send + Sync {
    /// Returns a map of image IDs to their reference counts.
    ///
    /// The reference count indicates how many times each image is referenced in posts.
    /// If an image ID is not present in the map, it means that the image is not referenced at all
    /// (i.e., reference count is 0).
    async fn get_reference_counts(
        &self,
        image_ids: &[i32],
    ) -> Result<HashMap<i32, i64>, ImageError>;

    async fn is_image_referenced(&self, image_id: i32) -> Result<bool, ImageError> {
        let reference_counts = self.get_reference_counts(&[image_id]).await?;
        Ok(reference_counts.get(&image_id).copied().unwrap_or(0) > 0)
    }
}
