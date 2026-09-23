use crate::domain::entity::user::User;

pub struct UserMapper {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}

impl From<UserMapper> for User {
    fn from(val: UserMapper) -> Self {
        User {
            id: val.id,
            issuer: val.issuer,
            source_id: val.source_id,
            displayed_name: val.displayed_name,
            email: val.email,
        }
    }
}

impl From<User> for UserMapper {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            issuer: user.issuer,
            source_id: user.source_id,
            displayed_name: user.displayed_name,
            email: user.email,
        }
    }
}
