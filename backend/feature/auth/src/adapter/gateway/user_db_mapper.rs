use crate::domain::entity::user::User;

pub struct UserMapper {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
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

impl UserMapper {
    pub fn into_entity(self) -> User {
        User {
            id: self.id,
            issuer: self.issuer,
            source_id: self.source_id,
            displayed_name: self.displayed_name,
            email: self.email,
        }
    }
}
