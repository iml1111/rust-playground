use actix_web:: {
    HttpResponseBuilder,
    dev::ServiceResponse,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    http::header::{CONTENT_TYPE, HeaderValue},
    body::MessageBody,
    Result,
};
use super::response;

pub fn init<B: MessageBody + 'static>(error_handlers: ErrorHandlers<B>) -> ErrorHandlers<B> {
    error_handlers
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::METHOD_NOT_ALLOWED, method_not_allowed)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_server_error)
}

pub fn not_found<B: MessageBody + 'static>(
    res: ServiceResponse<B>
) -> Result<ErrorHandlerResponse<B>> {
    log::error!("404 Error Hooked!");
    get_default_error_response(res, "not_found".to_string())
}

pub fn method_not_allowed<B: MessageBody + 'static>(
    res: ServiceResponse<B>
) -> Result<ErrorHandlerResponse<B>> {
    log::error!("405 Error Hooked!");
    get_default_error_response(res, "method_not_found".to_string())
}

pub fn internal_server_error<B: MessageBody + 'static>(
    res: ServiceResponse<B>
) -> Result<ErrorHandlerResponse<B>> {
    log::error!("500 Error Hooked!");
    get_default_error_response(res, "internal_server_error".to_string())
}

fn get_default_error_response<B: MessageBody + 'static>(
    res: ServiceResponse<B>, 
    msg: String
) -> Result<ErrorHandlerResponse<B>> {
    let error_response = response::GenernalError{
        msg: msg,
        detail: None,
    };
    let (req, _) = res.into_parts();
    let res = HttpResponseBuilder::new(StatusCode::NOT_FOUND)
        .insert_header((CONTENT_TYPE, HeaderValue::from_static("application/json")))
        .json(error_response);
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
} 