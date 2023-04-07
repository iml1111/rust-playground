use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Deserialize, Serialize)]
pub struct GenernalError {
    pub msg: String,
    pub detail: Option<String>,
}

pub mod json_response {
    use super::{Response, GenernalError};
    use actix_web::{HttpResponse, http::header::ContentType};
    use serde::Serialize;

    pub fn ok<T: Serialize>(data: T) -> HttpResponse {
        let res = Response {
            msg: "ok".to_string(),
            data: Some(data)
        };
        let res = serde_json::to_string(&res).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res)
    }

    pub fn created<T: Serialize>(data: T) -> HttpResponse {
        let res = Response {
            msg: "created".to_string(),
            data: Some(data)
        };
        let res = serde_json::to_string(&res).unwrap();
        HttpResponse::Created()
            .content_type(ContentType::json())
            .body(res)
    }

    pub fn bad_request(detail: Option<String>) -> HttpResponse {
        let res = GenernalError {
            msg: "bad_request".to_string(),
            detail
        };
        let res = serde_json::to_string(&res).unwrap();
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(res)
    }

    pub fn forbidden(detail: Option<String>) -> HttpResponse {
        let res = GenernalError {
            msg: "forbidden".to_string(),
            detail
        };
        let res = serde_json::to_string(&res).unwrap();
        HttpResponse::Forbidden()
            .content_type(ContentType::json())
            .body(res)
    }
}