mod config;
mod app;
mod controller;
mod model;
mod tests;

use actix_web::{
    web,
    HttpServer, App,
    web::Data,
    http::KeepAlive, 
    middleware::{Logger, Compress, ErrorHandlers},
};
use env_logger::{init_from_env, Env};
use config::AppConfig;
use app::error_handler;
use app::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::from_env(".env");
    let ext_app_config = app_config.clone();
    init_from_env(Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        log::info!("Constructing the App:{}", app_config.app_name);
        App::new()
            .app_data(Data::new(app_config.clone()))
            // Middlewares
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(error_handler::init(ErrorHandlers::new()))
            // Routers
            .service(router::template::index)
            .service(web::scope("/api/v1").configure(router::v1::init))

    })
    .keep_alive(KeepAlive::Os) 
    .workers(ext_app_config.worker_num)
    .bind(("0.0.0.0", 8080))?
    .run().await
}

