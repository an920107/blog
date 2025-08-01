use sqlx::FromRow;

use crate::adapter::gateway::user_db_mapper::UserMapper;

#[derive(FromRow)]
pub struct UserRecord {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}

impl UserRecord {
    pub fn into_mapper(self) -> UserMapper {
        UserMapper {
            id: self.id,
            issuer: self.issuer,
            source_id: self.source_id,
            displayed_name: self.displayed_name,
            email: self.email,
        }
    }
}
