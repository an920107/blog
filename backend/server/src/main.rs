use actix_session::{
    SessionMiddleware, config::SessionMiddlewareBuilder, storage::RedisSessionStore,
};
use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use auth::framework::web::auth_web_routes::configure_auth_routes;
use image::framework::web::image_web_routes::configure_image_routes;
use openidconnect::reqwest;
use post::framework::web::post_web_routes::configure_post_routes;
use server::{configuration::Configuration, container::Container};
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to create HTTP client");

    let configuration = Configuration::new(http_client.clone()).await;

    let host = configuration.server.host.clone();
    let port = configuration.server.port;

    let db_pool = configuration.db.create_connection().await;
    let session_key = configuration.session.session_key.clone();
    let session_store = configuration.session.create_session_store().await;

    HttpServer::new(move || {
        create_app(
            db_pool.clone(),
            http_client.clone(),
            SessionMiddleware::builder(session_store.clone(), session_key.clone()),
            configuration.clone(),
        )
    })
    .bind((host, port))?
    .run()
    .await
}

fn create_app(
    db_pool: Pool<Postgres>,
    http_client: reqwest::Client,
    session_middleware_builder: SessionMiddlewareBuilder<RedisSessionStore>,
    configuration: Configuration,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new(db_pool, http_client, configuration);

    App::new()
        .wrap(session_middleware_builder.build())
        .app_data(web::Data::from(container.auth_controller))
        .app_data(web::Data::from(container.image_controller))
        .app_data(web::Data::from(container.post_controller))
        .configure(configure_auth_routes)
        .configure(configure_image_routes)
        .configure(configure_post_routes)
}
