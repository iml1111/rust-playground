
mod config;
mod app;

use actix_web::{
    HttpServer, App, Responder, HttpResponse,
    web,
    web::Data,
    http::KeepAlive, 
    middleware::{Logger, Compress, ErrorHandlers},
};
use env_logger::{init_from_env, Env};
use config::AppConfig;
use app::error_handler;
use app::response;

async fn hello(data: Data<AppConfig>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Welcome to {app_name}!")
}

async fn not_found_test() -> impl Responder {
    format!("Welcome!")
}


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
            .wrap(error_handler::init(ErrorHandlers::new()))
            // TODO: Routers
            .route("/", web::get().to(hello))
            .route("/404", web::get().to(not_found_test))
            // https://github.dev/emreyalvac/actix-web-jwt/
            // https://github.dev/XAMPPRocky/tokei_rs
    })
    .keep_alive(KeepAlive::Os) 
    .workers(4)
    .bind(("127.0.0.1", 5000))?
    .run().await
}

