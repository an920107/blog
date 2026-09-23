use std::sync::Arc;

use image::application::gateway::image_repository::ImageRepository;

use crate::{
    application::service::image_extractor::ImageExtractor,
    domain::{entity::post_info::PostInfo, error::post_error::PostError},
};

/// Attaches the preview image's MIME type and size to posts. The metadata lives in the image
/// feature, so it cannot be read as part of the post query; consumers such as the RSS feed need
/// it to emit a valid `<enclosure>`.
pub struct PreviewImageMetadataService;

impl PreviewImageMetadataService {
    pub async fn attach(
        image_repository: Arc<dyn ImageRepository>,
        posts: &mut [PostInfo],
    ) -> Result<(), PostError> {
        // Resolve each id once so the second pass only has to look the metadata up.
        let preview_image_ids: Vec<Option<i32>> = posts
            .iter()
            .map(|post| ImageExtractor::extract_preview_image_id(&post.preview_image_url))
            .collect();
        let image_ids: Vec<i32> = preview_image_ids.iter().flatten().copied().collect();

        if image_ids.is_empty() {
            return Ok(());
        }

        let image_meta_data = image_repository
            .get_image_meta_data_by_ids(&image_ids)
            .await
            .map_err(|e| PostError::Unexpected(anyhow::anyhow!(e.to_string())))?;

        for (post, image_id) in posts.iter_mut().zip(preview_image_ids) {
            if let Some(meta_data) = image_id.and_then(|id| image_meta_data.get(&id)) {
                post.preview_image_mime_type = Some(meta_data.mime_type.clone());
                post.preview_image_size = meta_data.size;
            }
        }

        Ok(())
    }
}
