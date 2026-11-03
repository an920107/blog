use chrono::NaiveDateTime;

pub struct CreatePostDbModel {
    pub semantic_id: String,
    pub title: String,
    pub description: String,
    pub preview_image_url: Option<String>,
    pub content: String,
    pub published_time: Option<NaiveDateTime>,
    pub label_ids: Vec<i32>,
    pub image_ids: Vec<i32>,
}

pub struct UpdatePostDbModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub preview_image_url: Option<String>,
    pub content: String,
    pub published_time: Option<NaiveDateTime>,
    pub label_ids: Vec<i32>,
    pub image_ids: Vec<i32>,
}
