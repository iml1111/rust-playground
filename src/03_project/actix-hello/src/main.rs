use actix_web::{
    web, App, HttpResponse, HttpServer, Responder, http::KeepAlive, Result,
    body::BoxBody, http::header::ContentType, HttpRequest,
    http::{header, StatusCode}, dev
};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers, Logger};
use env_logger::Env;
use serde::{Deserialize, Serialize};

struct AppState {
    app_name: String,
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello world! {app_name}")
}

async fn hello_json() -> impl Responder {
    MyObj { name: "IML" }
}

// https://actix.rs/docs/application#configure
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// impl Responder가 아니라 Result<String>처럼 직접 명시 가능.
async fn path_api(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("{}, {}", friend, user_id))
}

// String으로만 해도 되는듯
async fn query_api(query: web::Query<Info>) -> String {
    // 다른 쿼리가 들어와도 용인되긴 함.
    format!("Query: {}", query.username)
}

async fn json_body_api(json_body: web::Json<Info>) -> Result<String> {
    Ok(format!("Body: {}", json_body.username))
}

async fn form_body_api(form: web::Form<Info>) -> Result<String> {
    // x-www-form-urlencoded
    Ok(format!("Body: {}", form.username))
}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // https://actix.rs/docs/middleware#logging
            .wrap(Logger::default())
            // https://actix.rs/docs/middleware#error-handlers
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
            )
            .app_data(
                web::Data::new(
                    AppState {
                        app_name: String::from("Actix Web by IML"),
                    }
                )
            )
            // /app
            .configure(config) 
            .service(
                web::scope("/api")
                    // /api/test
                    .configure(scoped_config)
                    // /api/{user_id}/{friend}
                    .route("/{user_id}/{friend}", web::get().to(path_api))
                    // /api/query?username=asd
                    .route("/query", web::get().to(query_api))
                    // /api/json
                    .route("/json", web::post().to(json_body_api))
                    // /api/form
                    .route("/form", web::post().to(form_body_api))
            ) 
            .route("/", web::get().to(hello))
            .route("/hello-json", web::get().to(hello_json))
            .service(web::resource("/error").route(web::get().to(HttpResponse::InternalServerError)))

    })
    .keep_alive(KeepAlive::Os) // https://actix.rs/docs/server#keep-alive
    .workers(4) // https://actix.rs/docs/server#multi-threading
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}