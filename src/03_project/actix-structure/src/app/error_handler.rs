use actix_web:: {
    HttpResponse,
    dev::ServiceResponse,
    body::BoxBody,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    http::header::{CONTENT_TYPE, HeaderValue},
    web::Bytes,
    Result
};
use serde_json::json;

pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "not_found");
    Ok(ErrorHandlerResponse::Response(res.map_into_right_body(response)))
}

fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    res.response_mut()
        .headers_mut()
        .insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
    HttpResponse::build(res.status())
        .body(Bytes::from(json!({"msg": error})))
}