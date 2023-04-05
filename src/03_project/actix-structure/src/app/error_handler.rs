use actix_web:: {
    dev::ServiceResponse,
    body::{EitherBody, BoxBody},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    http::header::{CONTENT_TYPE, HeaderValue},
    Result,
};
use serde_json::json;

pub fn init<B: 'static>(error_handlers: ErrorHandlers<B>) -> ErrorHandlers<B> {
    error_handlers
        .handler(StatusCode::NOT_FOUND, not_found)
}

pub fn not_found<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    log::error!("not_found: {:?}", res.request().path());
    let (req, res) = res.into_parts();
    let res = res.set_body(r#"{"msg": "not_found"}"#.to_owned());
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}