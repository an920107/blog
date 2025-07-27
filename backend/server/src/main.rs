use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use image::framework::web::image_web_routes::configure_image_routes;
use post::framework::web::post_web_routes::configure_post_routes;
use server::container::Container;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = init_database().await;
    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "static".to_string());

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    HttpServer::new(move || create_app(db_pool.clone(), storage_path.clone()))
        .bind((host, port))?
        .run()
        .await
}

async fn init_database() -> Pool<Postgres> {
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

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    sqlx::migrate!("../migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    db_pool
}

fn create_app(
    db_pool: Pool<Postgres>,
    storage_path: String,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new(db_pool, &storage_path);

    App::new()
        .app_data(web::Data::from(container.post_controller))
        .app_data(web::Data::from(container.image_controller))
        .configure(configure_post_routes)
        .configure(configure_image_routes)
}
