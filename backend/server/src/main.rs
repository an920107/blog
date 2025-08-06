use actix_session::{
    SessionMiddleware, config::SessionMiddlewareBuilder, storage::RedisSessionStore,
};
use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    rt::Runtime,
    web,
};
use auth::framework::web::auth_web_routes::configure_auth_routes;
use image::framework::web::image_web_routes::configure_image_routes;
use openidconnect::reqwest;
use post::framework::web::post_web_routes::configure_post_routes;
use server::{
    api_doc::configure_api_doc_routes, configuration::Configuration, container::Container,
};
use sqlx::{Pool, Postgres};

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to create HTTP client");

    let rt = Runtime::new().unwrap();
    let configuration = rt.block_on(async { Configuration::new(http_client.clone()).await });

    let _guard = sentry::init((
        configuration.sentry.dsn.clone(),
        configuration.sentry.options.clone(),
    ));

    actix_web::rt::System::new().block_on(async {
        let host = configuration.server.host.clone();
        let port = configuration.server.port;

        let db_pool = configuration.db.create_connection().await;
        let session_key = configuration.session.session_key.clone();
        let session_store = configuration.session.create_session_store().await;

        HttpServer::new(move || {
            create_app(
                db_pool.clone(),
                http_client.clone(),
                sentry::integrations::actix::Sentry::builder()
                    .capture_server_errors(true)
                    .start_transaction(true),
                SessionMiddleware::builder(session_store.clone(), session_key.clone()),
                configuration.clone(),
            )
        })
        .bind((host, port))?
        .run()
        .await
    })?;

    Ok(())
}

fn create_app(
    db_pool: Pool<Postgres>,
    http_client: reqwest::Client,
    sentry_builder: sentry::integrations::actix::SentryBuilder,
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
        // The middlewares are executed in opposite order as registration.
        .wrap(sentry_builder.finish())
        .wrap(session_middleware_builder.build())
        .app_data(web::Data::from(container.auth_controller))
        .app_data(web::Data::from(container.image_controller))
        .app_data(web::Data::from(container.post_controller))
        .configure(configure_api_doc_routes)
        .configure(configure_auth_routes)
        .configure(configure_image_routes)
        .configure(configure_post_routes)
}
