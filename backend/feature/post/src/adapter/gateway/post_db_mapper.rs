use crate::{adapter::gateway::post_info_db_mapper::PostInfoMapper, domain::entity::post::Post};

pub struct PostMapper {
    pub id: i32,
    pub info: PostInfoMapper,
    pub content: String,
}

impl From<PostMapper> for Post {
    fn from(val: PostMapper) -> Self {
        Post {
            id: val.id,
            info: val.info.into(),
            content: val.content,
        }
    }
}
