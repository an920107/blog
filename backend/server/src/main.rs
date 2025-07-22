use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use post::framework::web::post_web_routes::configure_post_routes;
use server::container::Container;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = init_database().await;

    HttpServer::new(move || create_app(db_pool.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn init_database() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".to_string());

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
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new(db_pool);

    App::new()
        .app_data(web::Data::from(container.post_controller))
        .configure(configure_post_routes)
}
