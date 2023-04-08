use std::env;

static _AUTHOR: &'static str = "IML";

pub trait IConfig {
    fn from(filename: String) -> Self;
}

#[derive(Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub worker_num: usize,
    pub contact_url: String,
    pub slow_api_time: f32,
}

impl AppConfig {
    pub fn from_env(filename: &str) -> AppConfig {
        dotenv::from_filename(filename).ok();

        let app_name: String = env::var("APP_NAME")
            .unwrap_or("IMActor".to_string());

        let worker_num: String = env::var("WORKER_NUM")
            .unwrap_or("1".to_string());
        let worker_num: usize = worker_num.parse().unwrap();
        
        let contact_url: String = env::var("CONTACT_URL")
            .unwrap_or("github.com/iml1111".to_string());

        let slow_api_time: String = env::var("SLOW_API_TIME")
            .unwrap_or("0.5".to_string());
        let slow_api_time: f32 = slow_api_time.parse().unwrap();

        AppConfig {
            app_name,
            worker_num,
            contact_url,
            slow_api_time
        }
    }
}
