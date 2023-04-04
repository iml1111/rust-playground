
mod config;

use config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let app_config = AppConfig::from(".env");
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info")
    );

    Ok(())
}
