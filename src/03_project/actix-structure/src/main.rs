
mod config;
mod app;

use actix_web::{
    HttpServer, App,
    web::Data,
    http::KeepAlive, 
    middleware::{Logger, Compress, ErrorHandlers},
    http::StatusCode,
};
use env_logger::{init_from_env, Env};
use config::AppConfig;
use app::error_handler;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::from_env(".env");
    init_from_env(Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        log::info!("Constructing the App:{}", app_config.app_name);
        App::new()
            .app_data(Data::new(app_config.clone()))
            // Middlewares
            .wrap(Logger::default())
            .wrap(Compress::default())
            // TODO: Error Handlers
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::NOT_FOUND,
                        error_handler::not_found)
            )
            // TODO: Routers 
            // https://github.dev/emreyalvac/actix-web-jwt/
    })
    .keep_alive(KeepAlive::Os) 
    .workers(4)
    .bind(("127.0.0.1", 5000))?
    .run().await
}

