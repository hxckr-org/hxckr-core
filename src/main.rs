use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use app::{
    auth::middleware::AuthMiddleware,
    routes,
    websockets::{handler::websocket_handler, manager::WebSocketManagerHandle},
};
use dotenvy::dotenv;
use env_logger::Env;
use service::{database::conn::get_connection_pool, queue::consume_queue};

mod app;
mod schema;
mod service;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    if std::env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL is not set");
    }
    if std::env::var("GIT_SERVICE_URL").is_err() {
        panic!("GIT_SERVICE_URL is not set");
    }
    if std::env::var("RABBITMQ_URL").is_err() {
        panic!("RABBITMQ_URL is not set");
    }
    if std::env::var("RABBITMQ_QUEUE_NAME").is_err() {
        panic!("RABBITMQ_QUEUE_NAME is not set");
    }

    let connection_url =
        std::env::var("CONNECTION_URL").unwrap_or_else(|_| "127.0.0.1:4925".to_string());
    let pool = get_connection_pool();
    let manager_handle = WebSocketManagerHandle::new();
    let manager_handle_clone = manager_handle.clone();

    tokio::spawn(async move {
        if let Err(e) = consume_queue(manager_handle_clone).await {
            log::error!("RabbitMQ consumer encountered an error: {:?}", e);
        }
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(manager_handle.clone()))
            .wrap(Logger::default())
            .wrap(AuthMiddleware)
            .wrap(cors)
            .service(web::scope("/api").configure(routes::init))
            .service(web::resource("/ws").route(web::get().to(websocket_handler)))
    })
    .bind(&connection_url)?
    .run()
    .await
}
