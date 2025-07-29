use std::env;

#[derive(Clone)]
pub struct StorageConfiguration {
    pub storage_path: String,
}

impl StorageConfiguration {
    pub fn new() -> Self {
        let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "static".to_string());
        Self { storage_path }
    }
}
