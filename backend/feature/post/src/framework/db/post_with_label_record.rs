use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct PostWithLabelRecord {
    pub post_id: i32,
    pub title: String,
    pub description: String,
    pub preview_image_url: String,
    pub content: String,
    pub published_time: Option<NaiveDateTime>,

    pub label_id: Option<i32>,
    pub label_name: Option<String>,
    pub label_color: Option<i64>,
}
