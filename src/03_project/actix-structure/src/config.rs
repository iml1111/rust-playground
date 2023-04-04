use std::env;

static AUTHOR = "IML";

pub trait IConfig {
    fn from(filename: &str) -> Self;
}

pub struct AppConfig {
    app_name: String,
    slow_api_time: f32,
}

impl IConfig for AppConfig {
    fn from(filename: &str) -> Self {
        dotenv::from_filename(filename).ok();
        let app_name: String = env::var("APP_NAME")
            .unwrap_or("IMActor".to_string());
        let slow_api_time: String = env::var("SLOW_API_TIME")
            .unwrap_or("0.5".to_string());
        let slow_api_time: f32 = s.parse().unwrap();

        AppConfig {
            app_name,
            slow_api_time
        }
    }
}
