use actix_web:: {
    dev::ServiceResponse,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    http::header::{CONTENT_TYPE, CONTENT_ENCODING, TRANSFER_ENCODING, HeaderValue},
    Result,
};
use serde_json::json;

pub fn init<B: 'static>(error_handlers: ErrorHandlers<B>) -> ErrorHandlers<B> {
    error_handlers
        .handler(StatusCode::NOT_FOUND, not_found)
}

pub fn not_found<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    log::error!("404 Error Hooked! Detected Path: {}", res.request().path());
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))

    // Empty Response 식별 불가능?
}