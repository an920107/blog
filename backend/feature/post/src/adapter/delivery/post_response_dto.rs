use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::post::Post;

use super::post_info_response_dto::PostInfoResponseDto;

#[derive(Serialize, ToSchema)]
pub struct PostResponseDto {
    pub id: i32,
    pub info: PostInfoResponseDto,
    pub content: String,
}

impl From<Post> for PostResponseDto {
    fn from(entity: Post) -> Self {
        Self {
            id: entity.id,
            info: PostInfoResponseDto::from(entity.info),
            content: entity.content,
        }
    }
}
