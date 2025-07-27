#[derive(sqlx::FromRow)]
pub struct ImageRecord {
    pub id: i32,
    pub mime_type: String,
}
