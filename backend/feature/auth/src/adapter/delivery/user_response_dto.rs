use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entity::user::User;

#[derive(Serialize, ToSchema)]
pub struct UserResponseDto {
    pub id: i32,
    pub displayed_name: String,

    #[schema(format = Email)]
    pub email: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        UserResponseDto {
            id: user.id,
            displayed_name: user.displayed_name,
            email: user.email,
        }
    }
}
