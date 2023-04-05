
mod config;
mod app;

use actix_web::{
    HttpServer, App, Responder, HttpResponse,
    web,
    web::Data,
    http::KeepAlive, 
    middleware::{Logger, Compress, ErrorHandlers},
    http::StatusCode,
};
use env_logger::{init_from_env, Env};
use config::AppConfig;
use app::error_handler;

async fn hello(data: Data<AppConfig>) -> impl Responder {
    let app_name = &data.app_name;
    //format!("Welcome to {app_name}!")

    // TODO: 어쩌면 이게 더 편할지도?
    HttpResponse::NotFound().json(r#"{"msg": "not_found"}"#.to_string())
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
            // TODO: Error Handlers
            .wrap(error_handler::init(ErrorHandlers::new()))
            // TODO: Routers
            .route("/", web::get().to(hello))
            // https://github.dev/emreyalvac/actix-web-jwt/
    })
    .keep_alive(KeepAlive::Os) 
    .workers(4)
    .bind(("127.0.0.1", 5000))?
    .run().await
}

