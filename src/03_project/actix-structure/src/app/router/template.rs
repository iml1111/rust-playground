use actix_web::{
    web::Data, Responder, get,
    HttpResponse
};
use crate::config::AppConfig;

#[get("/")]
pub async fn index(data: Data<AppConfig>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h1>Welcome to {}!</h1>", app_name))
}