use actix_web:: {
    HttpResponse,
    dev::ServiceResponse,
    body::{EitherBody, BoxBody},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    http::header::{CONTENT_TYPE, HeaderValue},
    web::Bytes,
    Result
};
use serde_json::json;

pub fn not_found<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "An error occurred";
    // Destructures ServiceResponse into request and response components
    let (req, res) = res.into_parts();
    // Create a new response with the modified body
    let res = res.set_body(error_message).map_into_boxed_body();
    // Create a new ServiceResponse with the modified response
    let res = ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}