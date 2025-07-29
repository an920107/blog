use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct DbConfiguration {
    pub database_url: String,
}

impl DbConfiguration {
    pub fn new() -> Self {
        let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
        let user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "".to_string());
        let dbname = env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());

        let encoded_password =
            percent_encoding::utf8_percent_encode(&password, percent_encoding::NON_ALPHANUMERIC)
                .to_string();

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, encoded_password, host, port, dbname
        );

        Self { database_url }
    }

    pub async fn create_connection(&self) -> Pool<Postgres> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
            .expect("Failed to create database connection pool");

        sqlx::migrate!("../migrations")
            .run(&db_pool)
            .await
            .expect("Failed to run database migrations");

        db_pool
    }
}
