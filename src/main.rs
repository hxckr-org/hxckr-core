use actix_web::{middleware::Logger, web, App, HttpServer};
use app::{
    auth::middleware::AuthMiddleware,
    routes,
    websockets::{handler::websocket_handler, manager::WebSocketManagerHandle},
};
use dotenvy::dotenv;
use env_logger::Env;
use service::database::conn::get_connection_pool;

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

    let pool = get_connection_pool();
    let manager_handle = WebSocketManagerHandle::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(manager_handle.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .wrap(AuthMiddleware)
                    .configure(routes::init),
            )
            .service(
                web::resource("/ws")
                    .wrap(AuthMiddleware)
                    .route(web::get().to(websocket_handler)),
            )
            .service(
                web::resource("/webhook").route(web::post().to(routes::webhook::handle_webhook)),
            )
    })
    .bind("127.0.0.1:4925")?
    .run()
    .await
}
