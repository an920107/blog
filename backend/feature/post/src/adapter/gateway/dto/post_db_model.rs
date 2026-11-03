use crate::{
    adapter::gateway::dto::post_info_db_model::PostInfoDbModel, domain::entity::post::Post,
};

pub struct PostDbModel {
    pub id: i32,
    pub info: PostInfoDbModel,
    pub content: String,
}

impl From<PostDbModel> for Post {
    fn from(val: PostDbModel) -> Self {
        Post {
            id: val.id,
            info: val.info.into(),
            content: val.content,
        }
    }
}
