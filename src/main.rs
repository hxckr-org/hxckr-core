use actix_web::{middleware::Logger, web, App, HttpServer};
use app::auth::middleware::AuthMiddleware;
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

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(AuthMiddleware)
            .wrap(Logger::default())
            .service(web::scope("/api").configure(app::routes::init))
    })
    .bind("127.0.0.1:4925")?
    .run()
    .await
}
