use std::env;

static _AUTHOR: &'static str = "IML";

pub trait IConfig {
    fn from(filename: String) -> Self;
}

#[derive(Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub contact_url: String,
    pub slow_api_time: f32,
}

impl AppConfig {
    pub fn from_env(filename: &str) -> AppConfig {
        dotenv::from_filename(filename).ok();

        let app_name: String = env::var("APP_NAME")
            .unwrap_or("IMActor".to_string());
        
        let contact_url: String = env::var("CONTACT_URL")
            .unwrap_or("github.com/iml1111".to_string());

        let slow_api_time: String = env::var("SLOW_API_TIME")
            .unwrap_or("0.5".to_string());
        let slow_api_time: f32 = slow_api_time.parse().unwrap();

        AppConfig {
            app_name,
            contact_url,
            slow_api_time
        }
    }
}
