#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub issuer: String,
    pub source_id: String,
    pub displayed_name: String,
    pub email: String,
}
