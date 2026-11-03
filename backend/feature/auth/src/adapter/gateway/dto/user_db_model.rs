use crate::domain::entity::user::User;

pub struct UserDbModel {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}

impl From<UserDbModel> for User {
    fn from(val: UserDbModel) -> Self {
        User {
            id: val.id,
            issuer: val.issuer,
            source_id: val.source_id,
            displayed_name: val.displayed_name,
            email: val.email,
        }
    }
}

impl From<User> for UserDbModel {
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
