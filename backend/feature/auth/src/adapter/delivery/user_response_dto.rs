use serde::Serialize;

use crate::domain::entity::user::User;

#[derive(Serialize)]
pub struct UserResponseDto {
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        UserResponseDto {
            source_id: user.source_id,
            displayed_name: user.displayed_name,
            email: user.email,
        }
    }
}
