use crate::application::error::post_error::PostError;

use super::post_info::PostInfo;

pub struct Post {
    pub id: i32,
    pub info: PostInfo,
    pub content: String,
}

impl Post {
    pub fn validate(&self) -> Result<(), PostError> {
        self.info.validate()?;
        Ok(())
    }
}
