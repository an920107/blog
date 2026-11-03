use sqlx::FromRow;

use crate::adapter::gateway::dto::user_db_model::UserDbModel;

#[derive(FromRow)]
pub struct UserRecord {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}

impl UserRecord {
    pub fn into_db_model(self) -> UserDbModel {
        UserDbModel {
            id: self.id,
            issuer: self.issuer,
            source_id: self.source_id,
            displayed_name: self.displayed_name,
            email: self.email,
        }
    }
}
