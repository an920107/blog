use crate::{adapter::gateway::post_info_mapper::PostInfoMapper, domain::entity::post::Post};

pub struct PostMapper {
    pub id: i32,
    pub info: PostInfoMapper,
    pub content: String,
}

impl PostMapper {
    pub fn to_entity(&self) -> Post {
        Post {
            id: self.id,
            info: self.info.to_entity(),
            content: self.content.clone(),
        }
    }
}
